use bevy::prelude::*;

use crate::PlayBackState;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_ui)
            .add_systems(Update, button_changed_state);
    }
}

const DEFAULT_BG_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_BG_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn button_changed_state(
    mut query: Query<(&Interaction, &ButtonAction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<PlayBackState>,
) {
    for (interaction, action, mut bg) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg = PRESSED_BG_COLOR.into();
                match action {
                    ButtonAction::Play => {
                        state.paused = false;
                    }
                    ButtonAction::Pause => {
                        state.paused = true;
                    }
                    _ => {}
                }
            },
            Interaction::Hovered => *bg = HOVERED_BG_COLOR.into(),
            Interaction::None => *bg = DEFAULT_BG_COLOR.into(),
        };
        info!("Playback state {:?}", state);
    }
}

#[derive(Component, Debug)]
enum ButtonAction {
    Play,
    Pause,
    Step,
}

#[derive(Bundle)]
struct PlaybackButtonBundle {
    button: ButtonBundle,
    action: ButtonAction,
}

impl Default for PlaybackButtonBundle {
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
            action: ButtonAction::Play,
        }
    }
}

fn initialize_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(PlaybackButtonBundle {
                    action: ButtonAction::Play,
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
                    action: ButtonAction::Step,
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
                    action: ButtonAction::Pause,
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
}
