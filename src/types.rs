#[derive(Debug)]
pub struct WindowInformation {
    pub time: u64,
    pub title: String,
    pub class: Vec<String>,
    pub execpath: String,
    /// URL of the active tab in the browser. Only set if the window is a browser window.
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Browser {
    Chrome,
    Opera,
    Brave,
    Edge,
    Firefox,
    Safari,
    Other,
}

impl WindowInformation {
    pub fn get_browser_type(&self) -> Option<Browser> {
        for class_name in &self.class {
            let class_name = class_name.to_lowercase();
            if class_name.contains("chrome") {
                return Some(Browser::Chrome);
            } else if class_name.contains("firefox") {
                return Some(Browser::Firefox);
            } else if class_name.contains("opera") {
                return Some(Browser::Opera);
            } else if class_name.contains("brave") {
                return Some(Browser::Brave);
            } else if class_name.contains("msedge") {
                return Some(Browser::Edge);
            }
        }

        None
    }
}
