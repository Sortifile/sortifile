use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use crate::functions::system::get_appdata_dir;
use tauri::Manager;
use tauri::path::BaseDirectory;

#[tauri::command]
pub async fn generate_rules(app: tauri::AppHandle, form_response: &str, model_name: &str) -> Result<(), String> {
    let appdata_dir = get_appdata_dir().unwrap();
    let generate_rules_command = app.shell().sidecar("generate_rules").unwrap()
        .args(&[
            app.path().resolve("resources/1_generate_rules/system_prompt.json", BaseDirectory::Resource).unwrap().as_os_str().to_str().unwrap(),
            form_response,
            app.path().resolve("resources/1_generate_rules/form_question.json", BaseDirectory::Resource).unwrap().as_os_str().to_str().unwrap(),
            model_name,
            ]);
    let (mut rx, mut _child) = generate_rules_command.spawn().expect("Failed to spawn sidecar");

    // Example: asynchronously reading sidecar's stdout
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line_bytes) = event {
                let line = String::from_utf8_lossy(&line_bytes);
                println!("Sidecar output: {}", line);
            }
        }
    });
    Ok(())
}

