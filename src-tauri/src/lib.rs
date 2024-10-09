use std::{
    env,
    io::{BufRead, BufReader, Write},
    process::{Child, Command, Stdio},
    sync::Mutex,
};

use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, connection: State<ServicesConnection>) -> String {

    let mut output = String::new();
    
    if let Ok(mut plugin) = connection.plugin.lock() {
        if let Some(stdin) = plugin.stdin.as_mut() {
            stdin.write_all(format!("{}", name).as_bytes()).expect("写入子进程数据错误!");
            stdin.flush().expect("刷新子进程 stdin 错误!");  // Ensure data is sent immediately
        }

        if let Some(stdout) = plugin.stdout.as_mut() {
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                let line = line.expect("读取子进程数据错误!");
                output.push_str(&line);
                break;  // Assuming we read only one line of response
            }
        }
    }

    output.trim().to_string()
}

struct ServicesConnection {
    plugin: Mutex<Child>,
}

impl Drop for ServicesConnection {
    fn drop(&mut self) {
        if let Ok(mut plugin) = self.plugin.lock() {
            plugin.kill().expect("释放插件管理子进程错误！");
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
