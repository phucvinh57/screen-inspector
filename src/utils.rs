use crate::types::Browser;

pub fn get_browser_active_tab_url(browser: Browser) -> Option<String> {
    get_session_folder_path(browser)
}

fn get_session_folder_path(browser: Browser) -> Option<String> {
    let user = whoami::username();
    let mut browser_data_path: Option<String> = None;
    let session_folder = "User Data\\Default\\Sessions";
    if browser == Browser::Edge {
        if !cfg!(windows) {
            browser_data_path = Some(format!(
                "C:\\Users\\{}\\AppData\\Local\\Microsoft\\Edge",
                user
            ));
        } 
    } else if browser == Browser::Chrome {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!(
                "C:\\Users\\{}\\AppData\\Local\\Google\\Chrome",
                user
            ));
        } else if cfg!(target_os = "linux") {
            browser_data_path = Some(format!("/home/{}/.config/google-chrome", user));
        } else if cfg!(target_os = "macos") {
            browser_data_path = Some(format!(
                "/Users/{}/Library/Application Support/Google/Chrome",
                user
            ));
        } 
    } else if browser == Browser::Firefox {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!(
                "C:\\Users\\{}\\AppData\\Roaming\\Mozilla\\Firefox",
                user
            ));
        } else if cfg!(target_os = "linux") {
            browser_data_path = Some(format!("/home/{}/.mozilla/firefox", user));
        } else if cfg!(target_os = "macos") {
            browser_data_path = Some(format!(
                "/Users/{}/Library/Application Support/Firefox",
                user
            ));
        } 
    } else if browser == Browser::Opera {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!(
                "C:\\Users\\{}\\AppData\\Roaming\\Opera Software\\Opera Stable",
                user
            ));
        } else if cfg!(target_os = "linux") {
            browser_data_path = Some(format!("/home/{}/.config/opera", user));
        } else if cfg!(target_os = "macos") {
            browser_data_path = Some(format!(
                "/Users/{}/Library/Application Support/com.operasoftware.Opera",
                user
            ));
        } 
    } else if browser == Browser::Brave {
        if cfg!(target_os = "windows") {
            browser_data_path = Some(format!(
                "C:\\Users\\{}\\AppData\\Local\\BraveSoftware\\Brave-Browser",
                user
            ));
        } else if cfg!(target_os = "linux") {
            browser_data_path = Some(format!("/home/{}/.config/BraveSoftware", user));
        } else if cfg!(target_os = "macos") {
            browser_data_path = Some(format!(
                "/Users/{}/Library/Application Support/BraveSoftware/Brave-Browser",
                user
            ));
        } 
    }
    if browser_data_path.is_none() {
        return None;
    }
    let browser_data_path = browser_data_path.unwrap();
    let session_path = format!("{}/{}", browser_data_path, session_folder);
    Some(session_path)
}