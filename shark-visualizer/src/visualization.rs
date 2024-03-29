use bevy::input::mouse::MouseMotion;
pub use bevy::{prelude::*, window::CursorGrabMode};
use shark::shader::{FragThree, Shader};

use crate::{
    shader_compiler::VisualizationExportsWrapper, user_config::RespawnLedsEvent, PlayBackState,
};

pub struct VisualizationPlugin;

impl Plugin for VisualizationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VisualizationState {
            exports: None,
            fps_timer: Timer::from_seconds(0.0, TimerMode::Once),
        })
        .add_systems(Startup, initialize_visualization)
        .add_systems(
            Update,
            (
                rotate_visualization,
                respawn_leds,
                update_leds.run_if(|pb: Res<PlayBackState>| !pb.paused),
                step_visualization,
            ),
        )
        .add_event::<StepEvent>();
    }
}

#[derive(Resource)]
pub struct VisualizationState {
    pub exports: Option<VisualizationExportsWrapper<'static, FragThree>>,
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

fn respawn_leds(
    mut commands: Commands,
    mut respawn_ev: EventReader<RespawnLedsEvent>,
    state: Res<VisualizationState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    root: Query<Entity, With<LedRoot>>,
    leds: Query<Entity, With<Led>>,
) {
    for _ in respawn_ev.read() {
        info!("Despawning all LEDs");
        for entity in leds.iter() {
            commands.entity(entity).despawn();
        }

        info!("Spawning LEDs");
        let led_root = root.single();
        for led in state.exports.as_ref().unwrap().points() {
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
                        transform: Transform::from_xyz(led.x as _, led.y as _, led.z as _),
                        material: materials.add(Color::BLACK.into()),
                        ..default()
                    },
                    Led,
                ))
                .set_parent(led_root);
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
        if state.exports.is_none() {
            info!("visualization not running because shader is not set");
            info!("Press the Compile button!");
            return;
        }

        let shader = state.exports.as_ref().unwrap();

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
                alpha_mode: AlphaMode::Opaque,
                ..default()
            };

            *material = materials.add(new_material);
        }
    }
}
