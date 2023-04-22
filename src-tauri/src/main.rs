#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use std::process::Command;


#[tauri::command]
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|_, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let output = Command::new("open")
                .arg("x-apple.systempreferences:com.apple.systempreferences.AppleIDSettings?iCloud")
                .output()
                .expect("failed to execute process");
                
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}