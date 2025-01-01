mod linux;
mod darwin;
mod windows;

#[cfg(target_os = "macos")]
pub use darwin::get_current_window_information;
#[cfg(target_os = "linux")]
pub use linux::get_current_window_information;
#[cfg(target_os = "windows")]
pub use windows::get_current_window_information;
