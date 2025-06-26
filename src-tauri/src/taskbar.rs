use tauri::{App, Manager};
use tauri::{tray::{MouseButton, MouseButtonState, TrayIconBuilder}, AppHandle, WindowEvent};
use windows::{
    Win32::UI::Shell::{APPBARDATA, SHAppBarMessage, ABM_GETTASKBARPOS},
    Win32::Foundation::RECT
};
pub struct TaskbarWin{
    rect: RECT,
}

impl TaskbarWin {

    pub fn update_rect() -> Option<Self> {
        unsafe {
            let mut abd = APPBARDATA::default();
            abd.cbSize = std::mem::size_of::<APPBARDATA>() as u32;
            let res = SHAppBarMessage(ABM_GETTASKBARPOS, &mut abd);
            if res != 0 {
                Some(TaskbarWin { rect: abd.rc })
            } else {
                None
            }
        }
    }
    pub fn get_position(&self, window_size : tauri::PhysicalSize<u32>) -> tauri::Position {
        let size = window_size.cast::<i32>();
        let x = self.rect.right - 10 - size.width ;
        let y = self.rect.top - size.height;
        println!("{:?}", self.rect);
        tauri::Position::Physical(tauri::PhysicalPosition { x, y })
    }
}
