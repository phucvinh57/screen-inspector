mod browser;
mod darwin;
mod device;
mod linux;
mod snss;
mod types;
mod windows;

pub use {device::*, types::WindowInformation};

#[cfg(target_os = "macos")]
pub use darwin::get_current_window_information;
#[cfg(target_os = "linux")]
pub use linux::get_current_window_information;
#[cfg(target_os = "windows")]
pub use windows::get_current_window_information;

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
    use {std::thread::sleep, std::time::Duration};

    #[test]
    fn test_get_current_window_information() {
        sleep(Duration::from_secs(2));
        let window_info = get_current_window_information().unwrap();
        debug!("{:?}", window_info);
    }

    #[test]
    fn test_get_mouse_position() {
        let mouse_pos = device::get_mouse_coords();
        assert!(mouse_pos.0 > 0 && mouse_pos.1 > 0);
    }
}
