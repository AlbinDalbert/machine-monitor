#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use wmi::{COMLibrary, WMIConnection, Variant};
use std::{slice::SliceIndex, collections::HashMap};

fn main() {

    println!("program started");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_temp, get_cpu_util, get_memory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_temp() -> String {
    println!("get_temp was invoked!!");
    let wmi = init_wmi_connection();
    
    let results: Vec<HashMap<String, Variant>> = wmi
    .raw_query(
        "SELECT * FROM Win32_PerfFormattedData_Counters_ThermalZoneInformation",
    )
    .unwrap();
    
    //let mut temps: Vec<f64> = vec![];
    let mut c_temp: f64 = 0.0;
    for data in results {
    
        let kelvin: f64 = match data.get("Temperature").unwrap() {
            Variant::UI4(val) => *val as f64,
            _ => 0.0,
        };
        c_temp = kelvin - 273.0;
    
    }
    c_temp.to_string()
}


#[tauri::command]
fn get_cpu_util() -> String {
    "temp-string".to_string()
}

#[tauri::command]
fn get_memory() -> String {
    "temp-string".to_string()
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

fn init_wmi_connection() -> WMIConnection{
    
    let com_lib = match COMLibrary::new() {
        Ok(com_lib) => com_lib,
        Err(_) => panic!("failed to initiate COMLibrary"),
    };

    let wmi_con = match WMIConnection::new(com_lib.into()){
        Ok(wmi_con) => wmi_con,
        Err(_) => panic!("Failed to initiate WMI Connection"),
    };

    wmi_con
}

