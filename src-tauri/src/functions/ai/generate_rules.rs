use crate::functions;
use crate::functions::ai::utils;
use crate::functions::file;
use crate::functions::zone;
use crate::functions::sql;
use glob::Pattern;
use serde_json::to_string;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tauri::command;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use crate::functions::system::get_appdata_dir;
use tauri::Manager;
use tauri::path::BaseDirectory;
use async_recursion::async_recursion;
use serde_json::{json, Value};
use std::time::SystemTime;
use chrono::{DateTime, Utc};

#[tauri::command]
pub fn ai_create_rule(
    app: tauri::AppHandle,
    zone_name: &str,
    zone_path: &str,
    create_from_structure: bool,
    form_response: &str,
) -> Result<(), String> {
    if create_from_structure {
        // For now, we call get_file_tree() and then later run prompt 5.
        // For example:
        // let tree = get_file_tree(zone_path);
        // generate_rules::generate_rules_from_tree(tree, form_response, zone_name);
    } else {
        // Write the form_response to a temporary file.
        functions::system::write_to_temp_file(
            format!("zone_{}_form_response_tmp.json", zone_name),
            form_response.to_string(),
        );
        let generate_rules_command = app
            .shell()
            .sidecar("generate_rules")
            .map_err(|e| e.to_string())?
            .args(&[
                app.path()
                    .resolve("resources/1_generate_rules/system_prompt.json", BaseDirectory::Resource)
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap(),
                app.path()
                    .resolve("resources/1_generate_rules/form_question.json", BaseDirectory::Resource)
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap(),
                format!("zone_{}_form_response_tmp.json", zone_name).as_str(),
                format!("zone_{}_rule_tmp.json", zone_name).as_str(),
            ]);
        let (mut rx, _child) = generate_rules_command.spawn().map_err(|e| e.to_string())?;
        // Read sidecar stdout asynchronously.
        tauri::async_runtime::spawn(async move {
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line_bytes) = event {
                    let line = String::from_utf8_lossy(&line_bytes);
                    println!("generate_rules sidecar output: {}", line);
                }
            }
        });
    }
    Ok(())
}