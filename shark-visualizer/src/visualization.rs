use bevy::input::mouse::MouseMotion;
pub use bevy::{prelude::*, window::CursorGrabMode};
use palette::LinSrgb;
use shark::shader::FragThree;

use crate::{
    user_config::{DespawnLedsEvent, SpawnLedsEvent, UserConfigState},
    PlayBackState,
};

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VisualizationState {
            shader: None,
            fps_timer: Timer::from_seconds(0.0, TimerMode::Once),
        })
        .add_systems(Startup, initialize_visualization)
        .add_systems(
            Update,
            (
                rotate_visualization,
                spawn_leds,
                despawn_leds,
                update_leds.run_if(|pb: Res<PlayBackState>| !pb.paused),
                step_visualization,
            ),
        )
        .add_event::<StepEvent>();
    }
}

#[derive(Resource)]
pub struct VisualizationState {
    pub shader:
        Option<Box<dyn shark::shader::Shader<FragThree, Output = LinSrgb<f64>> + Send + Sync>>,
    pub fps_timer: Timer,
}

#[derive(Component)]
struct LedRoot;

fn initialize_visualization(mut commands: Commands) {
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
    mut materials: ResMut<Assets<StandardMaterial>>,
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
                                radius: 0.05,
                                ..default()
                            }
                            .into(),
                        ),
                        transform: Transform::from_xyz(led.x, led.y, led.z),
                        material: materials.add(Color::BLACK.into()),
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

fn update_leds(
    mut state: ResMut<VisualizationState>,
    pb: Res<PlayBackState>,
    time: Res<Time>,
    mut step_writer: EventWriter<StepEvent>,
) {
    state
        .fps_timer
        .set_duration(std::time::Duration::try_from_secs_f32(1.0 / pb.fps).unwrap());
    state.fps_timer.tick(time.delta());
    if !state.fps_timer.finished() {
        return;
    }

    step_writer.send(StepEvent);

    state.fps_timer.reset();
}

#[derive(Event)]
pub struct StepEvent;

fn step_visualization(
    mut query: Query<(&mut Handle<StandardMaterial>, &Transform), With<Led>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    state: ResMut<VisualizationState>,
    time: Res<Time>,
    mut step_reader: EventReader<StepEvent>,
) {
    for _ in step_reader.read() {
        if state.shader.is_none() {
            info!("visualization not running because shader is not set");
            info!("Press the Compile button!");
            return;
        }

        let shader = state.shader.as_ref().unwrap();

        for (mut material, transform) in query.iter_mut() {
            let color = shader.shade(FragThree {
                pos: [
                    transform.translation.x as _,
                    transform.translation.y as _,
                    transform.translation.z as _,
                ],
                time: time.elapsed_seconds_f64(),
            });

            let color = Color::rgb_linear(color.red as _, color.green as _, color.blue as _);

            let new_material = StandardMaterial {
                base_color: Color::BLACK,
                emissive: color,
                alpha_mode: 
                    AlphaMode::Opaque,
                ..default()
            };

            *material = materials
                .add(new_material);
        }
    }
}
