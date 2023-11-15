#![feature(rustc_private)]
mod shader_compiler;
mod ui;
mod visualization;

use bevy::prelude::*;

extern crate rustc_driver;
extern crate rustc_interface;

pub use rustc_interface::{run_compiler, Config};

#[derive(Resource, Debug)]
struct PlayBackState {
    paused: bool,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ui::UiPlugin,
            visualization::VisualizationPlugin,
            shader_compiler::ShaderCompilerPlugin,
        ))
        .add_systems(Startup, camera_setup)
        .insert_resource(PlayBackState { paused: true })
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
