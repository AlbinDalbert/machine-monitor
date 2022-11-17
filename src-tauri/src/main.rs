#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// use std::{slice::SliceIndex, collections::HashMap};
// use std::{thread, time::Duration, sync::mpsc::{Sender}};
use std::{collections::HashMap, thread, time::{self, Duration}, sync::mpsc::{Sender, SyncSender}};
use std::sync::mpsc::channel;
use std::result::Result::Ok;
use qmstats::*;

fn main() {

    println!("program started");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

#[tauri::command]
async fn process() {
    let (tx, rx) = channel::<Measurement>();
    let sleep_dur = Duration::new(1, 0);

    init_measurement_thread(tx, sleep_dur);

    loop {
        
        let res = rx.recv().unwrap();

        println!("{res:?}");

    }
}

// #[tauri::command]
// fn init_process(app: App) {
//     println!("new thread");
//     std::thread::spawn(move || {
//         loop {
//             app.emit_all("new_temp", Payload { message: "Tauri is awesome!".into() }).unwrap();
//         }
//     });
// }



