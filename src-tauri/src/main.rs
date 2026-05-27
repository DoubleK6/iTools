#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hardware;
mod launcher;
mod logger;

use hardware::sn::{read_sn, write_sn};
use launcher::{launch_tool, get_tools};
use logger::write_log;

#[cfg(windows)]
fn is_admin() -> bool {
    use std::ffi::c_void;
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token_handle: HANDLE = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_ok() {
            let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
            let mut size: u32 = 0;
            let result = GetTokenInformation(
                token_handle,
                TokenElevation,
                Some(&mut elevation as *mut _ as *mut c_void),
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            );
            let _ = CloseHandle(token_handle);
            return result.is_ok() && elevation.TokenIsElevated != 0;
        }
    }
    false
}

#[cfg(windows)]
fn run_as_admin() {
    use std::os::windows::ffi::OsStrExt;
    use windows::core::w;
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;

    let exe_path = std::env::current_exe().expect("Failed to get exe path");
    let exe_path_wide: Vec<u16> = exe_path.as_os_str().encode_wide().chain(Some(0)).collect();

    unsafe {
        ShellExecuteW(
            None,
            w!("runas"),
            windows::core::PCWSTR(exe_path_wide.as_ptr()),
            None,
            None,
            SW_SHOW,
        );
    }
    std::process::exit(0);
}

fn main() {
    println!("Starting iTools...");
    match logger::init() {
        Ok(path) => println!("Log file: {}", path.display()),
        Err(e) => eprintln!("Failed to initialize logger: {}", e),
    }
    logger::info("Starting iTools");

    // Windows 提权检查
    #[cfg(windows)]
    {
        if !is_admin() {
            logger::info("Not running as admin, requesting elevation...");
            run_as_admin();
        }
        logger::info("Running as administrator");
    }

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
