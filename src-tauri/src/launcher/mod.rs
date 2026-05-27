use crate::logger;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    #[serde(default)]
    pub icon: String,
}

fn get_exe_dir() -> Result<std::path::PathBuf, String> {
    let exe_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?;
    exe_path
        .parent()
        .ok_or("Failed to get exe directory".to_string())
        .map(|p| p.to_path_buf())
}

#[tauri::command]
pub fn launch_tool(tool_path: String) -> Result<String, String> {
    let full_path = get_exe_dir()?.join(&tool_path);
    logger::info(format!(
        "Launching tool: {} -> {}",
        tool_path,
        full_path.display()
    ));

    if !full_path.exists() {
        logger::error(format!("Tool not found: {}", full_path.display()));
        return Err(format!("Tool not found: {}", tool_path));
    }

    Command::new(&full_path).spawn().map_err(|e| {
        logger::error(format!(
            "Failed to launch tool {}: {}",
            full_path.display(),
            e
        ));
        format!("Failed to launch tool: {}", e)
    })?;

    logger::info(format!("Tool launched: {}", full_path.display()));
    Ok(format!("Launched: {}", tool_path))
}

#[tauri::command]
pub fn get_tools() -> Result<Vec<ToolInfo>, String> {
    let exe_dir = get_exe_dir()?;
    let config_path = exe_dir.join("config/config.json");
    logger::info(format!("Loading tools from config: {}", config_path.display()));

    if !config_path.exists() {
        logger::warn(format!("Tool config not found: {}", config_path.display()));
        return Ok(vec![]);
    }

    let content = std::fs::read_to_string(&config_path).map_err(|e| {
        logger::error(format!(
            "Failed to read config.json {}: {}",
            config_path.display(),
            e
        ));
        format!("Failed to read config.json: {}", e)
    })?;

    let mut tools: Vec<ToolInfo> = serde_json::from_str(&content).map_err(|e| {
        logger::error(format!(
            "Failed to parse config.json {}: {}",
            config_path.display(),
            e
        ));
        format!("Failed to parse config.json: {}", e)
    })?;
    logger::info(format!("Parsed {} tools from config", tools.len()));

    for tool in &mut tools {
        if tool.icon.is_empty() {
            logger::info(format!("No icon configured for {}", tool.id));
            continue;
        }

        let configured_icon = tool.icon.clone();
        let icon_path = exe_dir.join(&configured_icon);
        logger::info(format!(
            "Resolving icon for {}: {} -> {}",
            tool.id,
            configured_icon,
            icon_path.display()
        ));

        if icon_path.exists() {
            tool.icon = icon_path.to_string_lossy().to_string();
            logger::info(format!("Icon found for {}: {}", tool.id, tool.icon));
        } else {
            logger::warn(format!(
                "Icon not found for {}: {}",
                tool.id,
                icon_path.display()
            ));
            tool.icon = String::new();
        }
    }

    logger::info("Tool loading finished");
    Ok(tools)
}
