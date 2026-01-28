use crate::app::actions::AppActions;

pub mod file_picker;
pub mod library;
mod task;

pub struct Runtime {
    tokio: tokio::runtime::Runtime,
    pub library: library::Library,
    pub file_picker: file_picker::FilePicker,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            tokio: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .build()
                .unwrap(),
            library: library::Library::default(),
            file_picker: file_picker::FilePicker::default(),
        }
    }
}

impl Runtime {
    pub fn update(&mut self, ctx: &egui::Context, actions: &AppActions) {
        self.file_picker.update(ctx, actions);
    }
}
