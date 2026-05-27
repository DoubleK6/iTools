use crate::logger;
use std::os::windows::process::CommandExt;
use std::process::Command;

fn get_exe_dir() -> Result<std::path::PathBuf, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    exe_path
        .parent()
        .ok_or("Failed to get exe directory".to_string())
        .map(|p| p.to_path_buf())
}

fn read_sn_value() -> Result<String, String> {
    logger::info("Reading BIOS serial number");
    let output = Command::new("powershell")
        .creation_flags(0x08000000)
        .args(["-Command", "(Get-CimInstance Win32_BIOS).SerialNumber"])
        .output()
        .map_err(|e| {
            logger::error(format!("Failed to execute PowerShell while reading SN: {}", e));
            format!("Failed to execute PowerShell: {}", e)
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        logger::error(format!("PowerShell failed while reading SN: {}", stderr));
        return Err(format!("Failed to read SN: {}", stderr));
    }

    let sn = String::from_utf8_lossy(&output.stdout).trim().to_string();
    logger::info(format!("Read BIOS serial number: {}", sn));
    Ok(sn)
}

#[tauri::command]
pub fn read_sn() -> Result<String, String> {
    read_sn_value()
}

#[tauri::command]
pub fn write_sn(sn: String) -> Result<String, String> {
    let old_sn = match read_sn_value() {
        Ok(value) => value,
        Err(e) => {
            logger::warn(format!("Unable to read old SN before write: {}", e));
            "<read failed>".to_string()
        }
    };

    let sidecar_path = get_exe_dir()?.join("sidecars/AMIDEWIN.EXE");
    logger::info(format!(
        "Starting SN write: old='{}', new='{}', tool={}",
        old_sn,
        sn,
        sidecar_path.display()
    ));

    if !sidecar_path.exists() {
        logger::error(format!("AMIDEWIN.exe not found: {}", sidecar_path.display()));
        return Err("AMIDEWIN.exe not found".to_string());
    }

    let output = Command::new(&sidecar_path)
        .args(["/SS", &sn])
        .output()
        .map_err(|e| {
            logger::error(format!("Failed to execute AMIDEWIN: {}", e));
            format!("Failed to execute AMIDEWIN: {}", e)
        })?;

    if output.status.success() {
        logger::info(format!("SN write succeeded: old='{}', new='{}'", old_sn, sn));
        Ok("SN written successfully".to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        logger::error(format!(
            "SN write failed: old='{}', new='{}', stderr={}",
            old_sn, sn, stderr
        ));
        Err(format!("AMIDEWIN failed: {}", stderr))
    }
}
