use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use rfd::FileDialog;

pub struct SystemFilePicker {
    // SystemFilePicker is not Send or Sync for compatibility with MacOS
    _marker: PhantomData<*const ()>,
}

impl SystemFilePicker {
    pub fn new_from_main_thread() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Prompt the user to pick a file.
    pub fn pick_file(&self, title: &str, directory: impl AsRef<Path>) -> Option<PathBuf> {
        FileDialog::new()
            .set_directory(directory)
            .set_title(title)
            .pick_folder()
    }
}
