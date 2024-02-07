#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::time::Duration;
use std::sync::mpsc::channel;
use std::result::Result::Ok;
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
    let sleep_dur = Duration::new(0,500000);
    init_measurement_thread(tx, sleep_dur);

    let mut total_memory: f64 = 0.0;
    let mut total_vram: f64 = 0.0;

    loop {
        
        let res = match rx.recv() {
            Ok(x) => x,
            Err(_) => break,
        };

        match res {
            Measurement::Temperature(x) => send_temp("temp", x, manager),
            Measurement::Memory(x) => send_memory("memory", total_memory - x, total_memory, manager),
            Measurement::CpuUtil(x) => send_cpu_util("cpu_util", x, manager),
            Measurement::TotalMemory(x) => total_memory = x,
            Measurement::VramTotal(x) => total_vram = x as f64,
            Measurement::VramUsed(x) => send_vram_usage("vram",  x as f64, total_vram, manager),
            Measurement::GpuUtil(x) => send_gpu_util("gpu_util", x, manager),
            Measurement::GpuTemp(x) => send_gpu_temp("gpu_temp", x, manager),
            x => eprintln!("undefined measurement: {:?}", x),
        }
    }
}

fn send_temp<R: tauri::Runtime>(event_id: &str, message: i32, manager: &impl Manager<R>) {
    let send = format!("{message}°C");
    match manager.emit_all(event_id, send) {
        Ok(_) => (),
        Err(_) => (),
    };
}

fn send_vram_usage<R: tauri::Runtime>(event_id: &str, message: f64, total_vram: f64, manager: &impl Manager<R>) {
    let vram = KiB_to_GiB(message);
    let total = KiB_to_GiB(total_vram);
    let send = format!("{vram:.2} /{total:.2}");
    match manager.emit_all(event_id, send) {
            Ok(_) => (),
            Err(_) => (),
    };
}

fn send_gpu_util<R: tauri::Runtime>(event_id: &str, message: u32, manager: &impl Manager<R>) {
    if message == 0 {
        return
    }
    let send = format!("{message}%");
    match manager.emit_all(event_id, send) {
            Ok(_) => (),
            Err(_) => (),
    };
}

fn send_gpu_temp<R: tauri::Runtime>(event_id: &str, message: u32, manager: &impl Manager<R>) {
    let send = format!("{message}°C");
    match manager.emit_all(event_id, send) {
        Ok(_) => (),
        Err(_) => (),
    };
}

fn send_memory<R: tauri::Runtime>(event_id: &str, message: f64, total_memory: f64, manager: &impl Manager<R>) {
    let memory = KiB_to_GiB(message);
    let total = KiB_to_GiB(total_memory);
    let send = format!("{memory:.2} /{total:.2}");
    match manager.emit_all(event_id, send) {
        Ok(_) => (),
        Err(_) => (),
    };
}

fn send_cpu_util<R: tauri::Runtime>(event_id: &str, message: f64, manager: &impl Manager<R>) {
    if message == 0.0 {
        return
    }
    let send = format!("{message}%");
    match manager.emit_all(event_id, send) {
            Ok(_) => (),
            Err(_) => (),
    };
}