use crate::app::actions::AppActions;

pub mod file_picker;
pub mod library;
mod task;

#[derive(Default)]
pub struct Runtime {
    pub library: library::Library,
    pub file_picker: file_picker::FilePicker,
}

impl Runtime {
    pub fn update(&mut self, ctx: &egui::Context, actions: &AppActions) {
        self.file_picker.update(ctx, actions);
    }
}
