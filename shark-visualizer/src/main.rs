mod shader_compiler;
mod ui;
mod user_config;
mod visualization;

use bevy::prelude::*;
use ui::system::SystemFilePicker;

#[derive(Resource, Debug)]
struct PlayBackState {
    paused: bool,
    fps: f32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ui::UiPlugin,
            visualization::VisualizationPlugin,
            shader_compiler::ShaderCompilerPlugin,
            user_config::UserConfigPlugin,
        ))
        .add_systems(Startup, camera_setup)
        .insert_resource(PlayBackState {
            paused: false,
            fps: 20.0,
        })
        .insert_non_send_resource(SystemFilePicker::new_from_main_thread())
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
