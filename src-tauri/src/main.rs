/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: <_>
 * File Name: main.rs
 * Author: B74Z3
 * Description: Entry Module for program
 ******************************************************************************/
/******************************************************************************
* Requirements
*
*
*
*
******************************************************************************/

/******************************************************************************
 * Input Variables:
 *   - <input_variable_1>: <description>
 *   - <input_variable_2>: <description>
 *   - ...
 ******************************************************************************/
/******************************************************************************
 * Output Variables:
 *   - <output_variable_1>: <description>
 *   - <output_variable_2>: <description>
 *   - ...
 ******************************************************************************/

/******************************************************************************
 * Libraries and Dependencies:
 *   - <library_1>: <description>
 *   - <library_2>: <description>
 *   - ...
 ******************************************************************************/

/******************************************************************************
 * Constants:
 ******************************************************************************/

/******************************************************************************
 * Structures and Enums:
 ******************************************************************************/

/******************************************************************************
 * Implementations:
 ******************************************************************************/

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod core;
use core::search_engine::{index_service::{build_index, search_system}, windows_service::{list_drives,open_folder}};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn run_startup_tasks() {
    build_index();
    println!("Running startup tasks...");
}

/******************************************************************************
 * Main Function:
 ******************************************************************************/

fn main() {
    tauri::Builder::default()
        .setup(|app|{
            run_startup_tasks();
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,search_system,list_drives,open_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
