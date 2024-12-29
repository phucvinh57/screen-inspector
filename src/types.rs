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
    Firefox,
    Opera,
    Brave,
    Edge,
    Other,
}

impl WindowInformation {
    pub fn get_browser_type(&self) -> Browser {
        for class_name in &self.class {
            let class_name = class_name.to_lowercase();
            if class_name.contains("chrome") {
                return Browser::Chrome;
            } else if class_name.contains("firefox") {
                return Browser::Firefox;
            } else if class_name.contains("opera") {
                return Browser::Opera;
            } else if class_name.contains("brave") {
                return Browser::Brave;
            } else if class_name.contains("msedge") {
                return Browser::Edge;
            }
        }

        Browser::Other
    }
}
