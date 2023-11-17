use std::path::PathBuf;

use bevy::prelude::*;

use crate::shader_compiler::{CompileShaderEvent, ShaderCompilerState};
use crate::visualization::StepEvent;
use crate::PlayBackState;

use self::system::{pick_file, SystemFilePicker};

pub mod system;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_ui)
            .add_systems(
                Update,
                (
                    playback_button_changed_state,
                    compile_button_changed_state,
                    theme_buttons,
                    handle_error,
                ),
            )
            .add_event::<ErrorMessageEvent>()
            .add_event::<ManifestPathSetEvent>();
    }
}

#[derive(Component)]
struct ErrorTimer {
    timer: Timer,
}

#[derive(Event)]
pub enum ErrorMessageEvent {
    ManifestPathNotSet,
    NotAWorkspace,
    CargoError(String),
    TooManyLibs,
    NoLibs,
    NoShaderExport,
    NoSharkToml,
    CouldntReadSharkToml,
    InvalidSharkToml,
    UnsupportedFrag,
}

fn handle_error(
    mut err_ev: EventReader<ErrorMessageEvent>,
    mut query: Query<(&mut Text, &mut ErrorTimer)>,
    time: Res<Time>,
) {
    let (mut text, mut err_timer) = query.single_mut();
    for ev in err_ev.read() {
        err_timer.timer.reset();
        err_timer.timer.unpause();
        match ev {
            ErrorMessageEvent::ManifestPathNotSet => {
                text.sections[0].value = "Manifest folder not set".to_string();
            }
            ErrorMessageEvent::NotAWorkspace => {
                text.sections[0].value =
                    "Specified folder doesn't contain 'Cargo.toml'".to_string();
            }
            ErrorMessageEvent::CargoError(err) => {
                text.sections[0].value = err.clone();
            }
            ErrorMessageEvent::TooManyLibs => {
                text.sections[0].value = "Crate output more than on library".to_string();
            }
            ErrorMessageEvent::NoLibs => {
                text.sections[0].value = "Crate output no libraries".to_string();
            }
            ErrorMessageEvent::NoShaderExport => {
                text.sections[0].value =
                    "Library has no shader export function, or it has the incorrect header, or your shark.toml contains incorrect metadata"
                        .to_string();
            }
            ErrorMessageEvent::NoSharkToml => {
                text.sections[0].value = "No shark.toml in the given path".to_string();
            }
            ErrorMessageEvent::CouldntReadSharkToml => {
                text.sections[0].value = "Couldn't read shark.toml in the given path".to_string();
            }
            ErrorMessageEvent::InvalidSharkToml => {
                text.sections[0].value = "Couldn't parse shark.toml, is it valid?".to_string();
            }
            ErrorMessageEvent::UnsupportedFrag => {
                text.sections[0].value = "Exported shader used unsupported fragment type. Only FragThree is supported currently".to_string();
            }
        }
    }

    err_timer.timer.tick(time.delta());
    if err_timer.timer.just_finished() {
        text.sections[0].value = "".to_string();
    }
}

const DEFAULT_BG_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BG_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn theme_buttons(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut bg) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => *bg = PRESSED_BG_COLOR.into(),
            Interaction::Hovered => *bg = HOVERED_BG_COLOR.into(),
            Interaction::None => *bg = DEFAULT_BG_COLOR.into(),
        };
    }
}

fn playback_button_changed_state(
    mut query: Query<(&Interaction, &PlaybackButtonAction), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<PlayBackState>,
    mut step_writer: EventWriter<StepEvent>,
) {
    for (interaction, action) in query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            match action {
                PlaybackButtonAction::Play => {
                    state.paused = false;
                }
                PlaybackButtonAction::Pause => {
                    state.paused = true;
                }
                PlaybackButtonAction::Step => {
                    step_writer.send(StepEvent);
                }
            }
        }
    }
}

#[derive(Event)]
pub struct ManifestPathSetEvent(pub PathBuf);

fn compile_button_changed_state(
    mut query: Query<(&Interaction, &CompileButtonAction), (Changed<Interaction>, With<Button>)>,
    mut comp_writer: EventWriter<CompileShaderEvent>,
    mut manifest_writer: EventWriter<ManifestPathSetEvent>,
    mut err_writer: EventWriter<ErrorMessageEvent>,
    mut compiler_state: ResMut<ShaderCompilerState>,
    picker: NonSend<SystemFilePicker>,
) {
    for (interaction, action) in query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            match action {
                CompileButtonAction::SetFilePath => {
                    let path = pick_file(&picker, "Select manifest root", "~");
                    if let Some(path) = path.clone() {
                        if !path.join("Cargo.toml").exists() {
                            err_writer.send(ErrorMessageEvent::NotAWorkspace);
                            continue;
                        }
                        if !path.join("shark.toml").exists() {
                            err_writer.send(ErrorMessageEvent::NoSharkToml);
                            continue;
                        }
                        manifest_writer.send(ManifestPathSetEvent(path))
                    }
                    compiler_state.manifest_folder = path;
                }
                CompileButtonAction::Compile => {
                    comp_writer.send(CompileShaderEvent);
                }
            }
        }
    }
}

#[derive(Bundle)]
pub struct ThemedButtonBundle {
    button: ButtonBundle,
}

impl Default for ThemedButtonBundle {
    fn default() -> Self {
        Self {
            button: ButtonBundle {
                style: Style {
                    width: Val::Px(100.0),
                    height: Val::Px(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.5, 0.5, 0.5).into(),
                ..default()
            },
        }
    }
}

#[derive(Component, Debug)]
enum PlaybackButtonAction {
    Play,
    Pause,
    Step,
}

#[derive(Bundle)]
struct PlaybackButtonBundle {
    button: ThemedButtonBundle,
    action: PlaybackButtonAction,
}

impl Default for PlaybackButtonBundle {
    fn default() -> Self {
        Self {
            action: PlaybackButtonAction::Play,
            button: default(),
        }
    }
}

#[derive(Component, Debug)]
enum CompileButtonAction {
    Compile,
    SetFilePath,
}

#[derive(Bundle)]
struct CompileButtonBundle {
    button: ThemedButtonBundle,
    action: CompileButtonAction,
}

impl Default for CompileButtonBundle {
    fn default() -> Self {
        Self {
            action: CompileButtonAction::Compile,
            button: default(),
        }
    }
}

fn initialize_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                column_gap: Val::Px(50.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(50.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(PlaybackButtonBundle {
                            action: PlaybackButtonAction::Play,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Play",
                                    TextStyle {
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::all(Val::Px(15.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn(PlaybackButtonBundle {
                            action: PlaybackButtonAction::Step,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Step",
                                    TextStyle {
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::all(Val::Px(15.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn(PlaybackButtonBundle {
                            action: PlaybackButtonAction::Pause,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Pause",
                                    TextStyle {
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::all(Val::Px(15.0)),
                                    ..default()
                                }),
                            );
                        });
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(50.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(CompileButtonBundle {
                            action: CompileButtonAction::SetFilePath,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "SetPath",
                                    TextStyle {
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::all(Val::Px(15.0)),
                                    ..default()
                                }),
                            );
                        });
                    parent
                        .spawn(CompileButtonBundle {
                            action: CompileButtonAction::Compile,
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    "Compile",
                                    TextStyle {
                                        font_size: 30.0,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::all(Val::Px(15.0)),
                                    ..default()
                                }),
                            );
                        });
                });

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::RED,
                            ..default()
                        },
                    ),
                    ..default()
                },
                ErrorTimer {
                    timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                },
            ));
        });
}
