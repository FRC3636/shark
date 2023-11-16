use bevy::input::mouse::MouseMotion;
pub use bevy::{prelude::*, window::CursorGrabMode};
use palette::LinSrgb;
use shark::shader::FragThree;

use crate::user_config::{DespawnLedsEvent, SpawnLedsEvent, UserConfigState};

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VisualizationState { shader: None })
            .add_systems(Startup, initialize_visualization)
            .add_systems(Update, (rotate_visualization, spawn_leds, despawn_leds));
    }
}

#[derive(Resource)]
pub struct VisualizationState {
    pub shader:
        Option<Box<dyn shark::shader::Shader<FragThree, Output = LinSrgb<f64>> + Send + Sync>>,
}

#[derive(Component)]
struct LedRoot;

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

    commands.spawn((LedRoot, GlobalTransform::default(), Transform::default()));
}

const SENSITIVITY: f32 = 0.0008;

fn rotate_visualization(
    mut windows: Query<&mut Window>,
    mut led_root: Query<&mut Transform, With<LedRoot>>,
    mut motions: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
) {
    let mut window = windows.single_mut();

    if buttons.pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;

        for motion in motions.read() {
            let mut led_root_transform = led_root.single_mut();
            led_root_transform.rotate_y(motion.delta.x * SENSITIVITY);
        }
    } else {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

#[derive(Component)]
pub struct Led;

fn spawn_leds(
    mut commands: Commands,
    mut spawn_ev: EventReader<SpawnLedsEvent>,
    user_config: Res<UserConfigState>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<Entity, With<LedRoot>>,
) {
    let led_root = query.single();

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
            commands
                .spawn((
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
                ))
                .set_parent(led_root);
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
