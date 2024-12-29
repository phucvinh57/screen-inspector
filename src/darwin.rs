#[cfg(target_os = "macos")]
use {super::types::WindowInformation, anyhow::Result};

#[cfg(target_os = "macos")]
pub fn get_current_window_information() -> Result<WindowInformation> {
    unimplemented!("Windows is not supported yet.")
}
