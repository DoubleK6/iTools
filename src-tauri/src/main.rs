#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hardware;
mod launcher;
mod logger;

use hardware::sn::{read_sn, write_sn};
use launcher::{launch_tool, get_tools};
use logger::write_log;

fn main() {
    println!("Starting iTools...");
    match logger::init() {
        Ok(path) => println!("Log file: {}", path.display()),
        Err(e) => eprintln!("Failed to initialize logger: {}", e),
    }
    logger::info("Starting iTools");

    match tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            read_sn,
            write_sn,
            launch_tool,
            get_tools,
            write_log
        ])
        .run(tauri::generate_context!())
    {
        Ok(_) => {
            logger::info("iTools closed normally");
            println!("iTools closed normally");
        }
        Err(e) => {
            logger::error(format!("Error running iTools: {}", e));
            eprintln!("Error running iTools: {}", e);
            std::process::exit(1);
        }
    }
}
