/******************************************************************************
 * Project Name: NeoExplorer
 * Package Name: system
 * File Name: tray.rs
 * Author: B74Z3
 * Description: This module handles opeartions like tray management etc.
 ******************************************************************************/

/******************************************************************************
 * Libraries:
 ******************************************************************************/

// Standard Libraries

// External Crates
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

// Internal Modules

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

pub fn create_system_tray(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
    let exit = MenuItemBuilder::with_id("exit", "Exit").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&exit]).build()?;
    let _tray = TrayIconBuilder::new()
        .icon(Image::from_path("icons/icon.png")?)
        .menu(&menu)
        .on_menu_event(move |app_handle, event| match event.id().as_ref() {
            "exit" => {
                let _ = app_handle.get_webview_window("main").unwrap().close();
            }
            _ => (),
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app_handle = tray.app_handle();
                #[cfg(not(target_os = "macos"))]
                {
                    if let Some(webview_window) = app_handle.get_webview_window("main") {
                        let _ = webview_window.show();
                        let _ = webview_window.set_focus();
                    }
                }

                #[cfg(target_os = "macos")]
                {
                    tauri::AppHandle::show(&app_handle.app_handle()).unwrap();
                }
            }
        })
        .build(app)?;
    Ok(())
}
