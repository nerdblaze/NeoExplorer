/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: ROOT
 * File Name: lib.rs
 * Author: B74Z3
 * Description: Entry Module for program
 ******************************************************************************/

mod core;
mod system;
mod utilities;

/******************************************************************************
 * Requirements
 *
 *
 *
 *
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries

// External Crates
use tauri::Manager;

// Internal Modules
use crate::core::{
    explorer_engine::{
        explorer_service::{
            create_file, create_folder, delete_file, delete_files, open_file, open_folder,
        },
        file_service::info_handler::get_file_info,
    },
    search_engine::{
        database_service::search_system,
        index_service::{build_index, list_drives},
    },
};
use crate::system::{tray::create_system_tray, window::create_new_window};

/******************************************************************************
 * Constants:
******************************************************************************/

/******************************************************************************
 * Structures and Enums:
 ******************************************************************************/

/******************************************************************************
 * Implementations:
 ******************************************************************************/

/******************************************************************************
 * Functions:
 ******************************************************************************/

#[tauri::command]
async fn run_startup_tasks() {
    println!("Running startup tasks...");
    build_index();
    println!("App is ready");
}

/******************************************************************************
 * Entrypoint Function:
 ******************************************************************************/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let app_handle = app.app_handle();
            create_system_tray(&app_handle).unwrap();
            tauri::async_runtime::spawn(async move {
                run_startup_tasks().await;
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            search_system,
            list_drives,
            open_folder,
            create_new_window,
            open_file,
            get_file_info,
            delete_file,
            delete_files,
            create_file,
            create_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
