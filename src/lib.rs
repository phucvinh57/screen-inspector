mod browser;
mod device;
mod native_app;
mod types;
use browser::get_browser_active_tab_url;
use log::debug;
pub use {device::*, types::WindowInformation};

pub fn get_current_window_information() -> Option<WindowInformation> {
    let mut window = native_app::get_current_window_information()?;

    let browser = window.get_browser_type();
    if let Some(b) = browser {
        debug!("Browser: {:?}", b);
        window.url = get_browser_active_tab_url(b);
    }

    Some(window)
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use {std::thread::sleep, std::time::Duration};

    #[test]
    fn test_get_current_window_information() {
        env_logger::init();
        sleep(Duration::from_secs(2));
        let window_info = get_current_window_information();
        assert!(window_info.is_some());
        println!("{:?}", window_info.unwrap());
    }

    #[test]
    fn test_get_mouse_position() {
        let mouse_pos = device::get_mouse_coords();
        assert!(mouse_pos.0 > 0 && mouse_pos.1 > 0);
    }
}
