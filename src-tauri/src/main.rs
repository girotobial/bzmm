// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mods;
mod settings;

use tauri_plugin_log::{log, Target, TargetKind};

use mods::handlers::get_enabled_mods;
use mods::{
    cancel_download, delete_mod, disable_mod, download_mod, enable_mod, get_downloaded_mods,
    get_mods, queue_download, update_mod,
};
use settings::{delete_profile, get_app_version, get_settings, update_profile, update_settings};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .clear_targets()
                .target(Target::new(TargetKind::LogDir { file_name: None }))
                .target(Target::new(TargetKind::Stdout))
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            update_profile,
            delete_profile,
            get_mods,
            get_downloaded_mods,
            get_enabled_mods,
            download_mod,
            queue_download,
            cancel_download,
            enable_mod,
            disable_mod,
            update_mod,
            delete_mod,
            get_app_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
