use tauri::{App, Manager};
use tauri::{tray::{MouseButton, MouseButtonState, TrayIconBuilder}, AppHandle, WindowEvent};
use crate::taskbar::TaskbarWin;



pub fn build_tray(app : AppHandle) -> tauri::Result<AppHandle>
{
     
    let _ = TrayIconBuilder::new()
        .on_tray_icon_event(|tray_handle, event| {
        tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
        match event {
            tauri::tray::TrayIconEvent::Click{
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            }=> {
                if let Some(window) = tray_handle.app_handle().get_webview_window("notmain") {
                        let _ = window.set_position(TaskbarWin::update_rect().unwrap().get_position(window.inner_size().unwrap()));
                        let _ = window.show();
                        let _ = window.set_focus();
                }
            }
            _ => {}
        }
        })    
    .build(&app)?;
    Ok(app)
}