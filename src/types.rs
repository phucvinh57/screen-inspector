#[derive(Debug)]
pub struct WindowInformation {
    pub  time: u64,
    pub title: String,
    pub class: Vec<String>,
    pub execpath: String,
    pub browser_url: Option<String>,
}
