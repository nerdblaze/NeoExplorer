/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: ROOT
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
mod system;
mod utilities;

use crate::core::explorer_engine::explorer_service::open_folder;
use crate::core::search_engine::{
    database_service::search_system,
    index_service::{build_index, list_drives},
};
use crate::system::tray::create_system_tray;

use tauri::Manager;

#[tauri::command]
async fn run_startup_tasks() {
    println!("Running startup tasks...");
    build_index();
    println!("App is ready");
}

/******************************************************************************
 * Main Function:
 ******************************************************************************/
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let app_handle = app.app_handle();
            create_system_tray(&app_handle)?;
            tauri::async_runtime::spawn(async move {
                run_startup_tasks().await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            search_system,
            list_drives,
            open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
