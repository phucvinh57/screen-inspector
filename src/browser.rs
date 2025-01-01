use crate::{snss, types::Browser};
use std::fs;

/// Get most recently modified session file from browser session folder
fn get_current_active_session_file(browser: Browser) -> Option<String> {
    let session_folder = get_session_folder_path(browser);

    let paths = fs::read_dir(session_folder?).ok()?;
    let mut latest_file: Option<(String, std::time::SystemTime)> = None;

    for path in paths {
        if let Ok(entry) = path {
            if !entry
                .path()
                .to_str()?
                .split('/')
                .last()?
                .starts_with("Session_")
            {
                continue;
            }
            let metadata = fs::metadata(entry.path()).ok()?;
            let modified = metadata.modified().ok()?;
            if latest_file.is_none() || modified > latest_file.as_ref()?.1 {
                latest_file = Some((entry.path().to_str()?.to_string(), modified));
            }
        }
    }

    latest_file.map(|(path, _)| path)
}

fn get_session_folder_path(browser: Browser) -> Option<String> {
    let binding = dirs::home_dir()?;
    let homedir = binding.to_str()?;
    let mut browser_data_path: Option<String> = None;
    let session_folder: Option<&str> = if cfg!(target_os = "windows") {
        Some("User Data\\Default\\Sessions")
    } else if cfg!(target_os = "linux") {
        Some("Default/Sessions")
    } else if cfg!(target_os = "macos") {
        Some("Default/Sessions")
    } else {
        return None;
    };
    if session_folder.is_none() {
        return None;
    }

    let session_folder = session_folder.unwrap();
    if browser == Browser::Edge {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!("{}\\AppData\\Local\\Microsoft\\Edge", homedir));
        }
    } else if browser == Browser::Chrome {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!("{}\\AppData\\Local\\Google\\Chrome", homedir));
        } else if cfg!(target_os = "linux") {
            browser_data_path = Some(format!("{}/.config/google-chrome", homedir));
        } else if cfg!(target_os = "macos") {
            browser_data_path = Some(format!(
                "{}/Library/Application Support/Google/Chrome",
                homedir
            ));
        }
    } else if browser == Browser::Firefox {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!("{}\\AppData\\Roaming\\Mozilla\\Firefox", homedir));
        } else if cfg!(target_os = "linux") {
            browser_data_path = Some(format!("{}/.mozilla/firefox", homedir));
        } else if cfg!(target_os = "macos") {
            browser_data_path = Some(format!("{}/Library/Application Support/Firefox", homedir));
        }
    } else if browser == Browser::Opera {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!(
                "{}\\AppData\\Roaming\\Opera Software\\Opera Stable",
                homedir
            ));
        } else if cfg!(target_os = "linux") {
            browser_data_path = Some(format!("{}/.config/opera", homedir));
        } else if cfg!(target_os = "macos") {
            browser_data_path = Some(format!(
                "{}/Library/Application Support/com.operasoftware.Opera",
                homedir
            ));
        }
    } else if browser == Browser::Brave {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!(
                "{}\\AppData\\Local\\BraveSoftware\\Brave-Browser",
                homedir
            ));
        } else if cfg!(target_os = "linux") {
            browser_data_path = Some(format!("{}/.config/BraveSoftware", homedir));
        } else if cfg!(target_os = "macos") {
            browser_data_path = Some(format!(
                "{}/Library/Application Support/BraveSoftware/Brave-Browser",
                homedir
            ));
        }
    } else if browser == Browser::Safari {
        if cfg!(target_os = "macos") {
            browser_data_path = Some(format!("{}/Library/Safari", homedir));
        }
    }
    if browser_data_path.is_none() {
        return None;
    }
    let browser_data_path = browser_data_path.unwrap();
    let session_path = format!("{}/{}", browser_data_path, session_folder);

    Some(session_path)
}

pub fn get_browser_active_tab_url(browser: Browser) -> Option<String> {
    let session_file = get_current_active_session_file(browser)?;
    snss::read_snss_file(session_file);
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use log::debug;

    #[test]
    fn test_get_browser_active_tab_url() {
        env_logger::init();
        let url = get_browser_active_tab_url(Browser::Chrome);
        assert!(url.is_none());
    }

    #[test]
    fn test_get_session_folder_path() {
        let url = get_session_folder_path(Browser::Chrome);
        assert!(url.is_some());
        #[cfg(target_os = "linux")]
        assert!(url.unwrap().ends_with("google-chrome/Default/Sessions"));
        #[cfg(target_os = "windows")]
        assert!(url
            .unwrap()
            .ends_with("Google\\Chrome\\User Data\\Default\\Sessions"));
        #[cfg(target_os = "macos")]
        assert!(url.unwrap().ends_with("Google/Chrome/Default/Sessions"));
    }

    #[test]
    fn test_get_current_active_session_file() {
        let file = get_current_active_session_file(Browser::Chrome);
        assert!(file.is_some());
        debug!("{:?}", file);
    }
}
