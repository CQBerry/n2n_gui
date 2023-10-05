// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[tauri::command]
fn start_n2n(
    server: &str,
    virtual_network_ip: &str,
    community_name: &str,
    encryption_key: &str,
    enable_logging: bool,
) -> (u32, String) {
    let mut n2n = Command::new("./edge");
    let creation_flag = if enable_logging { 0 } else { 0x08000000 };
    let basic_args = format!("-l {} -c {} -E -x 1", server, community_name);
    let optional_args = match (virtual_network_ip, encryption_key) {
        (_, "") => {
            format!("-a {}", virtual_network_ip)
        }
        ("", _) => {
            format!("-k {}", encryption_key)
        }
        _ => {
            format!("-a {} -k {}", virtual_network_ip, encryption_key)
        }
    };
    let args = format!("{} {}", basic_args, optional_args);
    let n2n = n2n
        .args(args.split(" "))
        .creation_flags(creation_flag)
        .spawn()
        .expect("An error occurred while running n2n");
    if virtual_network_ip == "" {
        thread::sleep(Duration::from_secs(8));
        if let Ok(interfaces) = get_if_addrs::get_if_addrs() {
            for interface in interfaces {
                let ip_addr = interface.ip().to_string();
                if ip_addr.starts_with("172") {
                    return (n2n.id(), ip_addr);
                }
            }
        };
    }
    (n2n.id(), virtual_network_ip.into())
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
