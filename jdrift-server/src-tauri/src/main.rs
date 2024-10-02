// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::sync::Mutex;
use serde::Serialize;
use tauri::{Manager, State};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(Debug, Serialize)]
pub struct Backend {
    pub port: u16
}

#[tauri::command]
fn backend(state: State<Mutex<Backend>>) -> Result<u16, ()> {
    Ok(state.lock().unwrap().port)
}

#[tauri::command]
fn open_devtools(window: tauri::Window) {
    window.open_devtools();
}

fn main() {
    let args: Box<[String]> = env::args().collect();
    let command_port: u16 = {
        let string = args.as_ref().get(1).unwrap_or(&"4417".to_string()).clone();
        string.parse().expect("failed to parse port argument")
    };

    println!("waiting for web-engine to request port {command_port}");

    tauri::Builder::default()
        .setup(move |app| {
            app.manage(Mutex::new(Backend { port: command_port }));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![backend, open_devtools])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
