#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// use std::{slice::SliceIndex, collections::HashMap};
// use std::{thread, time::Duration, sync::mpsc::{Sender}};
use std::{collections::HashMap, thread, time::{self, Duration}, sync::mpsc::{Sender, SyncSender}};
use std::sync::mpsc::channel;
use std::result::Result::Ok;
use std::fmt;
use tauri::Manager;
use qmstats::{KiB_to_GiB, Measurement, init_measurement_thread};

fn main() {

    println!("program started");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                update_process(&app_handle);
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}


fn update_process<R: tauri::Runtime>(manager: &impl Manager<R>) {

    let (tx, rx) = channel::<Measurement>();
    let sleep_dur = Duration::new(1, 500000);
    let assume = true;
    init_measurement_thread(tx, sleep_dur, assume);

    let mut total_memory: f64 = 0.0;

    loop {
        
        let res = rx.recv().unwrap();

        match res {
            Measurement::Temperature(x) => send_temp("temp", x, manager),
            Measurement::Memory(x) => send_memory("memory", total_memory - x, total_memory, manager),
            Measurement::CpuUtil(x) => send_cpu_util("cpu_util", x, manager),
            Measurement::TotalMemory(x) => total_memory = x,
            // Measurement::Temperature(x) => println!("temp {} C", x),
            // Measurement::Memory(x) => println!("memory {:.2} GB / {:.2} GB", total_memory - (KiB_to_GiB(x)), total_memory),
            // Measurement::CpuUtil(x) => println!("cpu {} %", x),
            // Measurement::TotalMemory(x) => total_memory = KiB_to_GiB(x),
            _ => println!("res is not temp"),
        }
        // println!();
    }
}

fn send_to_js<R: tauri::Runtime>(event_id: &str, message: String, manager: &impl Manager<R>) {
    // info!(?message, event_id);
    manager
        .emit_all(event_id, message)
        .unwrap();
}

fn send_temp<R: tauri::Runtime>(event_id: &str, message: f64, manager: &impl Manager<R>) {
    // info!(?message, event_id);
    let send = format!("{message}°C");
    manager
        .emit_all(event_id, send)
        .unwrap();
}


fn send_memory<R: tauri::Runtime>(event_id: &str, message: f64, total_memory: f64, manager: &impl Manager<R>) {
    // info!(?message, event_id);
    let memory = KiB_to_GiB(message);
    let total = KiB_to_GiB(total_memory);
    let send = format!("{memory:.2} GB / {total:.2} GB");
    manager
        .emit_all("memory", send)
        .unwrap();
}

fn send_cpu_util<R: tauri::Runtime>(event_id: &str, message: f64, manager: &impl Manager<R>) {
    // info!(?message, event_id);
    let send = format!("{message}%");
    manager
        .emit_all(event_id, send)
        .unwrap();
}