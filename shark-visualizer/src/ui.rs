use bevy::prelude::*;
use rfd::FileDialog;

use crate::PlayBackState;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_ui).add_systems(
            Update,
            (
                playback_button_changed_state,
                compile_button_changed_state,
                theme_buttons,
            ),
        );
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
                _ => {}
            }
        }

        info!("Playback state {:?}", state);
    }
}

fn compile_button_changed_state(
    mut query: Query<(&Interaction, &CompileButtonAction), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, action) in query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            match action {
                CompileButtonAction::SetFilePath => {
                    info!("Set file path");
                    let path = FileDialog::new()
                        .set_directory("~")
                        .add_filter("Rust", &["rs"])
                        .pick_file();
                    info!("Path {:?}", path);
                }
                CompileButtonAction::Compile => {
                    todo!();
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
        });
}
