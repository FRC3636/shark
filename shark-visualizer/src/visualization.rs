pub use bevy::prelude::*;

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_visualization);
    }
}

fn initialize_visualization(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PointLightBundle::default());

    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        ..default()
    });
}