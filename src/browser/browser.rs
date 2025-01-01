use log::warn;

use crate::types::Browser;
use std::fs;

use super::chromium;

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

    if browser == Browser::Edge {
        return if cfg!(target_os = "windows") {
            Some(format!(
                "{}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Sessions",
                homedir
            ))
        } else {
            warn!("Edge is only supported on Windows");
            None
        };
    }
    if browser == Browser::Chrome {
        return if cfg!(target_os = "windows") {
            Some(format!(
                "{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Sessions",
                homedir
            ))
        } else if cfg!(target_os = "linux") {
            Some(format!(
                "{}/.config/google-chrome/Default/Sessions",
                homedir
            ))
        } else if cfg!(target_os = "macos") {
            Some(format!(
                "{}/Library/Application Support/Google/Chrome/Default/Sessions",
                homedir
            ))
        } else {
            None
        };
    }
    if browser == Browser::Opera {
        return if cfg!(target_os = "windows") {
            Some(format!(
                "{}\\AppData\\Roaming\\Opera Software\\Opera Stable\\User Data\\Default\\Sessions",
                homedir
            ))
        } else if cfg!(target_os = "linux") {
            Some(format!("{}/.config/opera/Default/Sessions", homedir))
        } else if cfg!(target_os = "macos") {
            Some(format!(
                "{}/Library/Application Support/com.operasoftware.Opera",
                homedir
            ))
        } else {
            None
        };
    }
    if browser == Browser::Brave {
        return if cfg!(target_os = "windows") {
            Some(format!(
                "{}\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Session",
                homedir
            ))
        } else if cfg!(target_os = "linux") {
            Some(format!("{}/.config/BraveSoftware/Brave-Browser/Default/Session", homedir))
        } else if cfg!(target_os = "macos") {
            Some(format!(
                "{}/Library/Application Support/BraveSoftware/Brave-Browser",
                homedir
            ))
        } else {
            None
        };
    }
    // Firefox and Safari don't use Chromium session files.
    if browser == Browser::Firefox {
        return if cfg!(target_os = "windows") {
            Some(format!("{}\\AppData\\Roaming\\Mozilla\\Firefox", homedir))
        } else if cfg!(target_os = "linux") {
            let moz_folder = format!("{}/.mozilla/firefox", homedir);
            let content = fs::read_to_string(format!("{}/installs.ini", moz_folder)).ok()?;
            let default_profile = content.lines()
                .find(|line| line.starts_with("Default="))?
                .split('=')
                .nth(1)?
                .trim();
            Some(format!("{}/{}/sessionstore-backups", moz_folder, default_profile))
        } else if cfg!(target_os = "macos") {
            Some(format!("{}/Library/Application Support/Firefox", homedir))
        } else {
            None
        };
    }
    if browser == Browser::Safari {
        return if cfg!(target_os = "macos") {
            Some(format!("{}/Library/Safari", homedir))
        } else {
            None
        }
    }
    None
}

pub fn get_browser_active_tab_url(browser: Browser) -> Option<String> {
    let session_file = get_current_active_session_file(browser)?;
    chromium::get_current_active_url(session_file)
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
