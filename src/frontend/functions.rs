#[cfg(all(not(debug_assertions), windows))]
pub fn remove_windows_console() {
    unsafe {
        windows_sys::Win32::System::Console::FreeConsole();
    }
}
