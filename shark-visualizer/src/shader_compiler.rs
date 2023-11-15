use std::path::PathBuf;

use bevy::prelude::*;

use crate::ui::ErrorMessageEvent;

#[derive(Event)]
pub struct CompileShaderEvent;

#[derive(Resource)]
pub struct ShaderCompilerState {
    pub shader_path: Option<PathBuf>,
}

pub struct ShaderCompilerPlugin;
impl Plugin for ShaderCompilerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CompileShaderEvent>()
            .add_systems(Update, compile_shader)
            .insert_resource(ShaderCompilerState { shader_path: None });
    }
}

fn compile_shader(
    mut compile_ev: EventReader<CompileShaderEvent>,
    mut error_writer: EventWriter<ErrorMessageEvent>,
    state: Res<ShaderCompilerState>,
) {
    for _ in compile_ev.read() {
        match state.shader_path {
            Some(ref path) => info!("Compiling path: {:?}", path),
            None => error_writer.send(ErrorMessageEvent::ShaderPathNotSet),
        }
    }
}
