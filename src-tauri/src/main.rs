// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use randapp::conf::AppConf;
use tauri_plugin_log::{
    fern::colors::{Color, ColoredLevelConfig},
    LogTarget,
};

mod app;
use app::{setup};

fn main() {
    let mut log = tauri_plugin_log::Builder::default()
        .targets([LogTarget::Stdout, LogTarget::Webview])
        .level(log::LevelFilter::Trace);

    if cfg!(debug_assertions) {
        log = log.with_colors(ColoredLevelConfig {
            error: Color::Red,
            warn: Color::Yellow,
            debug: Color::Blue,
            info: Color::BrightGreen,
            trace: Color::Cyan,
        });
    }

    let _app_config = AppConf::read().write();

    tauri::Builder::default()
        .plugin(log.build())
        .invoke_handler(tauri::generate_handler![
            // generate_randnum,
            // reset,
            // return_list_number,
            // close_splashscreen,
            // init_list,
            // return_csv_path,
            // reload_csv_path
        ])
        .setup(setup::init)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
