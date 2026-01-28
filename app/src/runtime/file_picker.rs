use crate::app::actions::AppActions;
use egui::Context;
use egui_file_dialog::FileDialog;

#[derive(Debug, Copy, Clone)]
pub enum FilePickTarget {
    CreateLibrary,
    OpenLibrary,
}

#[derive(Debug, Clone)]
pub enum FilePickMode {
    Single,
    Multiple,
    Directory,
    Save { default_name: String },
}

#[derive(Default)]
pub struct FilePicker {
    dialog: FileDialog,
    target: Option<FilePickTarget>,
}

impl FilePicker {
    pub fn open(&mut self, target: FilePickTarget, mode: FilePickMode) {
        self.target = Some(target);
        match mode {
            FilePickMode::Single => self.dialog.pick_file(),
            FilePickMode::Multiple => self.dialog.pick_multiple(),
            FilePickMode::Directory => self.dialog.pick_directory(),
            FilePickMode::Save { default_name } => {
                self.dialog.config_mut().default_file_name = default_name;
                self.dialog.save_file();
            }
        }
    }

    pub fn open_single(&mut self, target: FilePickTarget) {
        self.open(target, FilePickMode::Single);
    }

    pub fn open_save(&mut self, target: FilePickTarget, default_name: impl Into<String>) {
        self.open(
            target,
            FilePickMode::Save {
                default_name: default_name.into(),
            },
        );
    }

    pub fn update(&mut self, ctx: &Context, actions: &AppActions) {
        let Some(target) = self.target else {
            return;
        };

        self.dialog.update(ctx);

        if let Some(path) = self.dialog.take_picked() {
            self.target = None;
            actions.file_picked(path, target);
        }

        if let Some(paths) = self.dialog.take_picked_multiple() {
            self.target = None;
            actions.files_picked(paths, target);
        }
    }
}
