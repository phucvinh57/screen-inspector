mod browser;
mod device;
mod native_app;
mod snss;
mod types;
pub use {device::*, types::WindowInformation};

pub fn get_current_window_information() -> Option<WindowInformation> {
    native_app::get_current_window_information()
}

#[cfg(test)]
mod tests {
    use super::*;
    use {std::thread::sleep, std::time::Duration};

    #[test]
    fn test_get_current_window_information() {
        sleep(Duration::from_secs(2));
        let window_info = get_current_window_information().unwrap();
        println!("{:?}", window_info);
    }

    #[test]
    fn test_get_mouse_position() {
        let mouse_pos = device::get_mouse_coords();
        assert!(mouse_pos.0 > 0 && mouse_pos.1 > 0);
    }
}
