use bevy::prelude::*;
use serde::Deserialize;

use crate::ui::{ErrorMessageEvent, ManifestPathSetEvent};

pub struct UserConfigPlugin;
impl Plugin for UserConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_user_config)
            .add_systems(Update, load_user_config)
            .add_event::<RespawnLedsEvent>();
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub visualization: Visualization,
}

#[derive(Deserialize, Debug)]
pub struct Visualization {
    pub fragment: FragType,

    pub exports_fn_identifier: String,
}

#[derive(Deserialize, Debug)]
pub enum FragType {
    FragOne,
    FragTwo,
    FragThree,
}

#[derive(Resource)]
pub struct UserConfigState {
    pub config: Option<Config>,
}

pub fn initialize_user_config(mut commands: Commands) {
    commands.insert_resource(UserConfigState { config: None })
}

#[derive(Event)]
pub struct RespawnLedsEvent;

pub fn load_user_config(
    mut ev_reader: EventReader<ManifestPathSetEvent>,
    mut err_writer: EventWriter<ErrorMessageEvent>,
    mut state: ResMut<UserConfigState>,
) {
    for ev in ev_reader.read() {
        let path = ev.0.join("shark.toml");
        if let Ok(toml) = std::fs::read_to_string(path) {
            match toml::from_str(toml.as_str()) {
                Ok(config) => {
                    state.config = Some(config);
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
