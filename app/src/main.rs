use crate::app::App;
use crate::directories::eframe_save_file_path;
use music_organizer_core::audio::identify::fingerprint::song_file_fingerprint;
use std::path::PathBuf;

mod app;
mod directories;
mod runtime;
mod widgets;

fn main() {
    let (fp, duration) = song_file_fingerprint(PathBuf::from("./music/aishite.opus")).unwrap();
    println!("Fingerprint: {:?}, duration: {:?}", fp, duration);

    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("Music Organizer")
            .with_app_id("io.github.zitronenjoghurt.music-organizer"),
        persist_window: true,
        persistence_path: Some(eframe_save_file_path()),
        ..Default::default()
    };

    eframe::run_native(
        "Music Organizer",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
