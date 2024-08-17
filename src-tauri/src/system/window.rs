/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: system
 * File Name: window.rs
 * Author: B74Z3
 * Description: This module handles opeartions like multi-windows etc.
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries
use std::{thread::sleep, time};

// External Crates
use rayon::spawn;
use tauri::{AppHandle, Emitter, EventTarget, WebviewUrl, WebviewWindowBuilder};

// Internal Modules
use super::WINDOW_COUNTER;

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
pub async fn create_new_window(
    app: AppHandle,
    folder_path: Option<String>,
    init_url: Option<String>,
) {
    spawn(move || {
        let binding = WINDOW_COUNTER.clone();
        let mut counter = binding.lock().unwrap();
        *counter += 1;
        let folder_path = folder_path.unwrap_or_else(|| "".to_string());
        let init_url = init_url.unwrap_or_else(|| "/".to_string());
        let new_label = format!("label-{}", counter);
        let new_window =
            WebviewWindowBuilder::new(&app, &new_label, WebviewUrl::App(init_url.into()))
                .decorations(false)
                .visible(false)
                .build()
                .unwrap();

        if !folder_path.is_empty() {
            sleep(time::Duration::from_millis(250));
            new_window
                .emit_to(EventTarget::labeled(&new_label), "initialize", folder_path)
                .unwrap();
        }
        let _ = new_window.show();
    });
}
