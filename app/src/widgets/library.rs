use crate::app::actions::AppActions;
use crate::runtime::file_picker::FilePickTarget;
use crate::runtime::Runtime;
use egui::{Response, Ui, Widget};

pub struct LibraryWidget<'a> {
    rt: &'a mut Runtime,
    actions: &'a AppActions,
}

impl<'a> LibraryWidget<'a> {
    pub fn new(rt: &'a mut Runtime, actions: &'a AppActions) -> Self {
        Self { rt, actions }
    }
}

impl Widget for LibraryWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Some(song_infos) = self.rt.library.song_infos() else {
            return ui
                .horizontal_centered(|ui| {
                    if ui.button("Open Library").clicked() {
                        self.rt.file_picker.open_single(FilePickTarget::OpenLibrary);
                    }

                    if ui.button("Create Library").clicked() {
                        self.rt
                            .file_picker
                            .open_save(FilePickTarget::CreateLibrary, "library.modb");
                    }
                })
                .response;
        };

        ui.label("Library")
    }
}
