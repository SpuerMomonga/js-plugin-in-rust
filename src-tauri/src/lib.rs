use std::{
    env,
    io::{Read, Write},
    process::{Child, Command, Stdio},
    sync::Mutex,
};

use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, connection: State<ServicesConnection>) -> String {
    if let Ok(mut plugin) = connection.plugin.lock() {
        if let Some(stdin) = plugin.stdin.as_mut() {
            let _ = stdin.write_all(name.as_bytes());
        }

        if let Some(stdout) = plugin.stdout.as_mut() {
            let mut output = String::new();
            stdout
                .read_to_string(&mut output)
                .expect("读取子进程数据错误!");
            return output;
        }
    }

    String::new()
}

struct ServicesConnection {
    plugin: Mutex<Child>,
}

impl Drop for ServicesConnection {
    fn drop(&mut self) {
        if let Ok(mut plugin) = self.plugin.lock() {
            let _ = plugin.kill();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let plugin_services = Command::new("node/node.exe")
        .arg("node/pluginServices.js")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start node process");

    tauri::Builder::default()
        .manage(ServicesConnection {
            plugin: Mutex::new(plugin_services),
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
