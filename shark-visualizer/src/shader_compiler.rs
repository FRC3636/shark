use std::path::PathBuf;

use bevy::prelude::*;
use palette::Oklab;

use crate::ui::ErrorMessageEvent;

#[derive(Event)]
pub struct CompileShaderEvent;

#[derive(Resource)]
pub struct ShaderCompilerState {
    pub manifest_folder: Option<PathBuf>,
}

pub struct ShaderCompilerPlugin;
impl Plugin for ShaderCompilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CompileShaderEvent>()
            .add_systems(Update, compile_shader)
            .insert_resource(ShaderCompilerState {
                manifest_folder: None,
            });
    }
}

fn compile_shader(
    mut compile_ev: EventReader<CompileShaderEvent>,
    mut error_writer: EventWriter<ErrorMessageEvent>,
    state: Res<ShaderCompilerState>,
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
                            unsafe {
                                let lib = libloading::Library::new(lib).unwrap();
                                info!("Loaded library: {:?}", lib);
                                // if let Ok(shader) =
                                //     lib.get::<libloading::Symbol<
                                //         unsafe extern "C" fn() -> shark::shader::VtableShader<
                                //             shark::shader::FragThree,
                                //             Oklab,
                                //         >,
                                //     >>(b"shader_export")
                                // {
                                //     info!("Loaded shader!");
                                // } else {
                                //     error_writer.send(ErrorMessageEvent::NoShaderExport)
                                // }
                            };
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

fn compile_from_manifest_root(path: &PathBuf) -> Result<Vec<PathBuf>, String> {
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
