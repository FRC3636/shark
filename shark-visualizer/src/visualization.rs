pub use bevy::prelude::*;

use crate::user_config::{DespawnLedsEvent, SpawnLedsEvent, UserConfigState};

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_visualization)
            .add_systems(Update, (spawn_leds, despawn_leds));
    }
}

fn initialize_visualization(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

#[derive(Component)]
pub struct Led;

fn spawn_leds(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnLedsEvent>,
    user_config: Res<UserConfigState>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for _ in spawn_ev.read() {
        for led in user_config
            .config
            .as_ref()
            .unwrap()
            .visualization
            .leds
            .leds
            .iter()
        {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(
                        shape::UVSphere {
                            radius: 0.1,
                            ..default()
                        }
                        .into(),
                    ),
                    transform: Transform::from_xyz(led.x, led.y, led.z),
                    ..default()
                },
                Led,
            ));
        }
    }
}

fn despawn_leds(
    mut commands: Commands,
    mut despawn_ev: EventReader<DespawnLedsEvent>,
    query: Query<Entity, With<Led>>,
) {
    for _ in despawn_ev.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
