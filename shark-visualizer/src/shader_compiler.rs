use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    sync::Mutex,
};

use bevy::prelude::*;
use palette::LinSrgb;
use rfd::FileDialog;
use shark::shader::{FragThree, Fragment, ShaderExport};

use crate::{
    ui::ErrorMessageEvent, user_config::UserConfigState, visualization::VisualizationState,
};

#[derive(Event)]
pub struct CompileShaderEvent;

#[derive(Resource)]
pub struct ShaderCompilerState {
    pub manifest_folder: Option<PathBuf>,
    lib: Option<libloading::Library>,
}

/// Provides access to APIs only available on the main thread.
#[derive(Resource)]
pub struct SystemUI {
    __private: PhantomData<()>,
}

impl SystemUI {
    fn new() -> Self {
        Self {
            __private: PhantomData,
        }
    }
}

pub struct ShaderCompilerPlugin;
impl Plugin for ShaderCompilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CompileShaderEvent>()
            .add_systems(Update, compile_shader)
            .insert_resource(ShaderCompilerState {
                manifest_folder: None,
                lib: None,
            })
            .insert_non_send_resource(SystemUI::new());
    }
}

pub struct ShaderExportWrapper<'a, F: Fragment + Send> {
    inner: Mutex<ShaderExport<'a, F>>,
}

impl<F: Fragment + Send> shark::shader::Shader<F> for ShaderExportWrapper<'static, F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let inner = self.inner.lock().unwrap();
        inner.shade(frag)
    }
}

fn compile_shader(
    mut compile_ev: EventReader<CompileShaderEvent>,
    mut error_writer: EventWriter<ErrorMessageEvent>,
    mut state: ResMut<ShaderCompilerState>,
    user_config: Res<UserConfigState>,
    mut visualization: ResMut<VisualizationState>,
) {
    for _ in compile_ev.read() {
        match state.manifest_folder {
            Some(ref path) => {
                info!("Compiling path: {:?}", path);
                match compile_from_manifest_root(path) {
                    Ok(paths) => {
                        if paths.len() > 1 {
                            error_writer.send(ErrorMessageEvent::TooManyLibs)
                        }
                        if let Some(lib) = paths.into_iter().next() {
                            let lib = unsafe { libloading::Library::new(lib).unwrap() };
                            info!("Loaded library: {:?}", lib);

                            let config = &user_config.config.as_ref().unwrap().visualization;

                            let symbol_name = config.shader_export_name.as_bytes().to_vec();

                            let shader: Box<
                                dyn shark::shader::Shader<FragThree, Output = LinSrgb<f64>>
                                    + Send
                                    + Sync,
                            > = unsafe {
                                match config.fragment {
                                    crate::user_config::FragType::FragOne
                                    | crate::user_config::FragType::FragTwo => {
                                        error_writer.send(ErrorMessageEvent::UnsupportedFrag);
                                        continue;
                                    }
                                    crate::user_config::FragType::FragThree => {
                                        if let Ok(func) = lib.get::<libloading::Symbol<
                                            unsafe extern "C" fn() -> ShaderExport<
                                                'static,
                                                FragThree,
                                            >,
                                        >>(
                                            &symbol_name
                                        ) {
                                            Box::new(ShaderExportWrapper {
                                                inner: Mutex::new(func()),
                                            })
                                        } else {
                                            error_writer.send(ErrorMessageEvent::NoShaderExport);
                                            continue;
                                        }
                                    }
                                }
                            };
                            info!("Successfully created shader!");
                            drop(visualization.shader.replace(shader));
                            info!("Replaced old shader");
                            drop(state.lib.replace(lib));
                            info!("Unloaded old library");
                        } else {
                            error_writer.send(ErrorMessageEvent::NoLibs)
                        }
                    }
                    Err(e) => error_writer.send(ErrorMessageEvent::CargoError(e)),
                }
            }
            None => error_writer.send(ErrorMessageEvent::ManifestPathNotSet),
        }
    }
}

fn compile_from_manifest_root(path: &Path) -> Result<Vec<PathBuf>, String> {
    let config = cargo::util::Config::default().unwrap();
    let workspace = cargo::core::Workspace::new(&path.join("Cargo.toml"), &config)
        .map_err(|e| e.to_string())?;

    let mut options =
        cargo::ops::CompileOptions::new(&config, cargo::core::compiler::CompileMode::Build)
            .map_err(|e| e.to_string())?;
    options.target_rustc_crate_types = Some(vec![String::from("cdylib")]);

    let comp = cargo::ops::compile(&workspace, &options).unwrap();
    let output_libs = comp.cdylibs.into_iter().map(|unit| unit.path).collect();

    Ok(output_libs)
}
