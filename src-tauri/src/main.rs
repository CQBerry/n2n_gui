// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

#[tauri::command]
fn start_n2n(
    server: &str,
    virtual_network_ip: &str,
    community_name: &str,
    encryption_key: &str,
    enable_logging: bool,
) -> u32 {
    let creation_flag = if enable_logging {
        0
    } else {
        0x08000000
    };
    if encryption_key != "" {
        let child = Command::new("edge.exe")
            .args([
                "-l",
                server,
                "-c",
                community_name,
                "-a",
                virtual_network_ip,
                "-k",
                encryption_key,
                "-E",
                "-x",
                "1",
            ])
            .creation_flags(creation_flag)
            .spawn()
            .expect("An error occurred while running n2n");
        child.id()
    } else {
        let child = Command::new("edge.exe")
            .args([
                "-l",
                server,
                "-c",
                community_name,
                "-a",
                virtual_network_ip,
                "-E",
                "-x",
                "1",
            ])
            .creation_flags(creation_flag)
            .spawn()
            .expect("An error occurred while running n2n");
        child.id()
    }
}

#[tauri::command]
fn kill_n2n(process_id: u32) {
    Command::new("cmd")
        .args(["/C", "taskkill", "/F", "/PID", &process_id.to_string()])
        .creation_flags(0x08000000)
        .spawn()
        .expect("An error occurred while killing n2n");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_n2n, kill_n2n])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
