use windows::core::w;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_DEFBUTTON1};

fn main() {
    unsafe {
        MessageBoxW(HWND::default(), w!("Message"), w!("Caption"), MB_DEFBUTTON1);
    }
}
