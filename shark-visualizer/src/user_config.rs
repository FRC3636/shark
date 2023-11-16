use bevy::prelude::*;
use serde::Deserialize;

use crate::ui::{ErrorMessageEvent, ManifestPathSetEvent};

pub struct UserConfigPlugin;
impl Plugin for UserConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_user_config)
            .add_systems(Update, load_user_config)
            .add_event::<SpawnLedsEvent>()
            .add_event::<DespawnLedsEvent>();
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub visualization: Visualization,
}

fn default_export_name() -> String {
    "shader_export".to_owned()
}

#[derive(Deserialize, Debug)]
pub struct Visualization {
    pub fragment: FragType,

    #[serde(default = "default_export_name")]
    pub shader_export_name: String,

    pub leds: Leds,
}

#[derive(Deserialize, Debug)]
pub enum FragType {
    FragOne,
    FragTwo,
    FragThree,
}

#[derive(Deserialize, Debug)]
pub struct Leds {
    pub leds: Vec<Led>,
}

#[derive(Deserialize, Debug)]
pub struct Led {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Resource)]
pub struct UserConfigState {
    pub config: Option<Config>,
}

pub fn initialize_user_config(mut commands: Commands) {
    commands.insert_resource(UserConfigState { config: None })
}

#[derive(Event)]
pub struct SpawnLedsEvent;

#[derive(Event)]
pub struct DespawnLedsEvent;

pub fn load_user_config(
    mut ev_reader: EventReader<ManifestPathSetEvent>,
    mut spawn_writer: EventWriter<SpawnLedsEvent>,
    mut despawn_writer: EventWriter<DespawnLedsEvent>,
    mut err_writer: EventWriter<ErrorMessageEvent>,
    mut state: ResMut<UserConfigState>,
) {
    for ev in ev_reader.read() {
        despawn_writer.send(DespawnLedsEvent);

        let path = ev.0.join("shark.toml");
        if let Ok(toml) = std::fs::read_to_string(path) {
            match toml::from_str(toml.as_str()) {
                Ok(config) => {
                    state.config = Some(config);
                    spawn_writer.send(SpawnLedsEvent)
                }
                Err(e) => {
                    error!("Error parsing shark.toml: {:?}", e);
                    err_writer.send(ErrorMessageEvent::InvalidSharkToml)
                }
            };
        } else {
            err_writer.send(ErrorMessageEvent::CouldntReadSharkToml);
        }
        info!("Config: {:?}", state.config);
    }
}
