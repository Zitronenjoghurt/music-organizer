use crate::app::actions::{AppAction, AppActions};
use crate::runtime::file_picker::FilePickTarget;
use crate::runtime::Runtime;
use crate::widgets::library::LibraryWidget;
use eframe::{Frame, Storage};
use egui::{CentralPanel, Context, FontDefinitions, SidePanel, TopBottomPanel, Widget};
use egui_notify::Toasts;
use std::path::PathBuf;

pub mod actions;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct App {
    #[serde(skip, default)]
    actions: AppActions,
    #[serde(skip, default)]
    rt: Runtime,
    #[serde(skip, default)]
    toasts: Toasts,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.egui_ctx.set_pixels_per_point(1.5);
        Self::setup_fonts(&cc.egui_ctx);
        cc.storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }

    fn setup_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| self.show_top_panel(ui));
        CentralPanel::default().show(ctx, |ui| self.show_central_panel(ui));
        SidePanel::left("side_panel").show(ctx, |ui| self.show_left_panel(ui));
        self.toasts.show(ctx);

        self.rt.update(ctx, &self.actions);
        for action in self.actions.take_actions() {
            if let Err(err) = self.handle_action(ctx, action) {
                self.toasts.error(err.to_string());
            }
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

// Rendering
impl App {
    fn show_top_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Music Organizer");
        });
    }

    fn show_central_panel(&mut self, ui: &mut egui::Ui) {}

    fn show_left_panel(&mut self, ui: &mut egui::Ui) {
        LibraryWidget::new(&mut self.rt, &self.actions).ui(ui);
    }
}

// Actions
impl App {
    fn handle_action(&mut self, ctx: &Context, action: AppAction) -> anyhow::Result<()> {
        match action {
            AppAction::FilePicked { path, target } => self.handle_file_picked(path, target)?,
            AppAction::FilesPicked { paths, target } => self.handle_files_picked(paths, target)?,
            AppAction::ToastError(message) => {
                self.toasts.error(message);
            }
            AppAction::ToastSuccess(message) => {
                self.toasts.success(message);
            }
            AppAction::ToastWarning(message) => {
                self.toasts.warning(message);
            }
        }
        Ok(())
    }

    fn handle_file_picked(&mut self, path: PathBuf, target: FilePickTarget) -> anyhow::Result<()> {
        match target {
            FilePickTarget::CreateLibrary => self.rt.library.attach(path)?,
            FilePickTarget::OpenLibrary => self.rt.library.attach(path)?,
        }
        Ok(())
    }

    fn handle_files_picked(
        &mut self,
        paths: Vec<PathBuf>,
        target: FilePickTarget,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
