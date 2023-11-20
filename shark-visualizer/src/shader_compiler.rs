use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use bevy::prelude::*;
use palette::LinSrgb;
use rand::Rng;

use shark::shader::{FragThree, Fragment};
use shark_visualizer_interface::VisualizationExports;

use crate::{
    ui::ErrorMessageEvent,
    user_config::{RespawnLedsEvent, UserConfigState},
    visualization::VisualizationState,
};

#[derive(Event)]
pub struct CompileShaderEvent;

#[derive(Resource)]
pub struct ShaderCompilerState {
    pub manifest_folder: Option<PathBuf>,
    lib_path: Option<PathBuf>,
    lib: Option<libloading::Library>,
}

pub struct ShaderCompilerPlugin;
impl Plugin for ShaderCompilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CompileShaderEvent>()
            .add_systems(Update, handle_compile_events)
            .insert_resource(ShaderCompilerState {
                manifest_folder: None,
                lib_path: None,
                lib: None,
            });
    }
}

pub struct VisualizationExportsWrapper<'a, F: Fragment + Send> {
    inner: Mutex<VisualizationExports<'a, F>>,
}
impl<F: Fragment + Send> VisualizationExportsWrapper<'_, F> {
    pub fn points(&self) -> Vec<shark::point::Point> {
        let inner = self.inner.lock().unwrap();
        inner.points.as_slice().to_vec()
    }
}

impl<F: Fragment + Send> shark::shader::Shader<F> for VisualizationExportsWrapper<'static, F> {
    type Output = LinSrgb<f64>;

    fn shade(&self, frag: F) -> Self::Output {
        let inner = self.inner.lock().unwrap();
        inner.shader.shade(frag)
    }
}

type ShaderExportFn<F> = unsafe extern "C" fn() -> VisualizationExports<'static, F>;

fn handle_compile_events(
    mut compile_ev: EventReader<CompileShaderEvent>,
    mut error_writer: EventWriter<ErrorMessageEvent>,
    mut state: ResMut<ShaderCompilerState>,
    user_config: Res<UserConfigState>,
    mut visualization: ResMut<VisualizationState>,
    mut respawn_writer: EventWriter<RespawnLedsEvent>,
) {
    for _ in compile_ev.read() {
        let library_path = match compile_shader(&state) {
            Err(e) => {
                error_writer.send(e);
                continue;
            }
            Ok(path) => {
                // TODO: this is a hack, maybe we can let cargo know about these files and clean somehow?
                if let Some(old) = state.lib_path.as_ref() {
                    std::fs::remove_file(old)
                        .unwrap_or_else(|_| warn!("Failed to remove old library"));
                }
                state.lib_path = Some(path.to_owned());

                path
            }
        };

        let lib = unsafe { libloading::Library::new(library_path).unwrap() };
        info!("Loaded library: {:?}", lib);

        let config = &user_config.config.as_ref().unwrap().visualization;

        let symbol_name = config.exports_fn_identifier.as_bytes().to_vec();

        let exports = unsafe {
            match config.fragment {
                // Small dimensional fragments are not supported yet
                crate::user_config::FragType::FragOne | crate::user_config::FragType::FragTwo => {
                    error_writer.send(ErrorMessageEvent::UnsupportedFrag);
                    continue;
                }
                crate::user_config::FragType::FragThree => {
                    let func: libloading::Symbol<ShaderExportFn<FragThree>> =
                        match lib.get(&symbol_name) {
                            Ok(func) => func,
                            Err(_) => {
                                error_writer.send(ErrorMessageEvent::NoShaderExport);
                                continue;
                            }
                        };

                    VisualizationExportsWrapper {
                        inner: Mutex::new(func()),
                    }
                }
            }
        };

        info!("Successfully created shader!");
        drop(visualization.exports.replace(exports));
        respawn_writer.send(RespawnLedsEvent);
        info!("Replaced old shader and led points");
        drop(state.lib.replace(lib));
        info!("Unloaded old library");
    }
}

fn compile_shader(state: &ShaderCompilerState) -> Result<PathBuf, ErrorMessageEvent> {
    if state.manifest_folder.is_none() {
        return Err(ErrorMessageEvent::ManifestPathNotSet);
    }
    let manifest_root = state.manifest_folder.as_ref().unwrap();

    let libs = match compile_from_manifest_root(manifest_root) {
        Ok(libs) => libs,
        Err(e) => return Err(ErrorMessageEvent::CargoError(e)),
    };

    if libs.len() > 1 {
        return Err(ErrorMessageEvent::TooManyLibs);
    }

    if let Some(lib_path) = libs.into_iter().next() {
        Ok(lib_path)
    } else {
        Err(ErrorMessageEvent::NoLibs)
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
    options.build_config.message_format = cargo::core::compiler::MessageFormat::Short;

    let comp = cargo::ops::compile(&workspace, &options).unwrap();
    let output_libs = comp
        .cdylibs
        .into_iter()
        .map(|unit| unit.path)
        // This is to avoid a bug (probably in libloading)
        // where the loaded Library is not changed after load and unload if it has the same filename both times
        // even though the file itself has changed
        .map(|path| {
            let output_name = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>();

            let mut new_path = path.clone();
            new_path.set_file_name(output_name);

            // just in case
            if let Some(ext) = path.extension() {
                new_path.set_extension(ext);
            }

            std::fs::rename(&path, &new_path).unwrap();
            info!(
                "Renamed {:?} to {:?}",
                path.file_name(),
                new_path.file_name()
            );
            new_path
        })
        .collect();

    Ok(output_libs)
}
