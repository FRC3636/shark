use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use bevy::ecs::system::Resource;
use rfd::FileDialog;

#[derive(Resource)]
pub struct SystemFilePicker {
    _private: PhantomData<()>,
}

impl SystemFilePicker {
    pub fn new_from_main_thread() -> Self {
        Self {
            _private: PhantomData,
        }
    }
}

/// Prompt the user to pick a file.
pub fn pick_file(
    _picker: &SystemFilePicker,
    title: &str,
    directory: impl AsRef<Path>,
) -> Option<PathBuf> {
    FileDialog::new()
        .set_directory(directory)
        .set_title(title)
        .pick_folder()
}
