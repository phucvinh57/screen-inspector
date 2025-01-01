#![allow(dead_code)]
// This lib is used to read SNSS file of browsers
use anyhow::Ok;
use log::debug;
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug)]
// TODO: Add more commands
enum SSNSCommand {
    SetTabWindow = 0,
    SetTabIndexInWindow = 2,
    UpdateTabNavigation = 6,
    SetSelectedNavigationIndex = 7,
    SetSelectedTabInIndex = 8,
    SetActiveWindow = 20,
    LastActiveTime = 21,
    SetTabGroup = 25,
    SetTabGroupMetadata2 = 27,
    TabClosed = 16,
    WindowClosed = 17,
}
impl SSNSCommand {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(SSNSCommand::SetTabWindow),
            2 => Some(SSNSCommand::SetTabIndexInWindow),
            6 => Some(SSNSCommand::UpdateTabNavigation),
            7 => Some(SSNSCommand::SetSelectedNavigationIndex),
            8 => Some(SSNSCommand::SetSelectedTabInIndex),
            20 => Some(SSNSCommand::SetActiveWindow),
            21 => Some(SSNSCommand::LastActiveTime),
            25 => Some(SSNSCommand::SetTabGroup),
            27 => Some(SSNSCommand::SetTabGroupMetadata2),
            16 => Some(SSNSCommand::TabClosed),
            17 => Some(SSNSCommand::WindowClosed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
struct TabHistory {
    index: u32,
    url: String,
    title: String,
}

#[derive(Clone, Debug)]
struct TabGroup {
    high: u64,
    low: u64,
    name: String,
}

#[derive(Clone, Debug)]
struct Tab {
    /// Index of the tab in the window (a relative index)
    index: u32,
    histories: Vec<TabHistory>,
    window_id: u32,
    deleted: bool,
    current_history_index: u32,
    group: Option<TabGroup>,
}

#[derive(Debug)]
struct Window {
    tabs: Vec<Tab>,
    active_tab_index: u32,
    active: bool,
    deleted: bool,
}

const SNSS_HEADER: [u8; 4] = [0x53, 0x4E, 0x53, 0x53];

fn read_u8<R: Read>(mut f: R) -> Option<u8> {
    let mut buf = [0; 1];
    f.read_exact(&mut buf).ok().unwrap();
    return Some(buf[0]);
}
fn read_u16<R: Read>(mut f: R) -> Option<u16> {
    let mut buf = [0; 2];
    f.read_exact(&mut buf).ok()?;
    Some(u16::from_le_bytes(buf))
}
fn read_u32<R: Read>(mut f: R) -> Option<u32> {
    let mut buf = [0; 4];
    f.read_exact(&mut buf).ok()?;
    Some(u32::from_le_bytes(buf))
}
fn read_u64<R: Read>(mut f: R) -> Option<u64> {
    let mut buf = [0; 8];
    f.read_exact(&mut buf).ok()?;
    Some(u64::from_le_bytes(buf))
}

fn read_string<R: Read>(mut f: R) -> Option<String> {
    let size = read_u32(&mut f)?;
    let mut rsize = size;
    if rsize % 4 != 0 {
        // Chrome 32 bit align pickled data
        rsize += 4 - rsize % 4;
    }
    let mut buf = vec![0; rsize as usize];
    f.read_exact(&mut buf).ok()?;
    Some(String::from_utf8_lossy(&buf).to_string())
}

fn read_string_16<R: Read>(mut f: R) -> Option<String> {
    let size = read_u32(&mut f)?;
    let mut rsize = size * 2;
    if rsize % 4 != 0 {
        // Chrome 32 bit align pickled data
        rsize += 4 - rsize % 4;
    }
    let mut buf: Vec<u8> = vec![0; rsize as usize];
    f.read_exact(&mut buf).ok()?;

    let buf16 = buf
        .chunks(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect::<Vec<u16>>();
    Some(String::from_utf16_lossy(&buf16).to_string())
}
fn read_raw<R: Read>(mut f: R, size: usize) -> anyhow::Result<Vec<u8>> {
    let mut buf = vec![0; size];
    f.read_exact(&mut buf)?;
    Ok(buf)
}

fn get_tab(tabs: &mut HashMap<u32, Tab>, tab_id: u32) -> &mut Tab {
    if tabs.contains_key(&tab_id) {
        tabs.get_mut(&tab_id).unwrap()
    } else {
        tabs.insert(
            tab_id,
            Tab {
                index: 0,
                histories: Vec::new(),
                window_id: 0,
                deleted: false,
                current_history_index: 0,
                group: None,
            },
        );
        tabs.get_mut(&tab_id).unwrap()
    }
}
fn get_window(windows: &mut HashMap<u32, Window>, window_id: u32) -> &mut Window {
    if windows.contains_key(&window_id) {
        windows.get_mut(&window_id).unwrap()
    } else {
        windows.insert(
            window_id,
            Window {
                tabs: Vec::new(),
                active_tab_index: 0,
                active: false,
                deleted: false,
            },
        );
        windows.get_mut(&window_id).unwrap()
    }
}
fn get_group(groups: &mut HashMap<String, TabGroup>, high: u64, low: u64) -> &mut TabGroup {
    let key = format!("{}|{}", high, low);
    if groups.contains_key(&key) {
        groups.get_mut(&key).unwrap()
    } else {
        groups.insert(
            key.clone(),
            TabGroup {
                high,
                low,
                name: String::new(),
            },
        );
        groups.get_mut(&key).unwrap()
    }
}

// SNSS file format. No \n seperator
// "SNSS" (0x534E5353): 4 bytes
// <version>: Int32 (4 bytes), should be 1 or 3
// These are the commands that are stored in the SNSS file
// <int16(size)><int8(type id)><payload(size - 1 bytes)>
// When user do an action, browser will append a "command" to SNSS file
pub fn read_snss_file(path: String) -> Option<String> {
    let mut f = File::open(path).ok()?;

    // Read header
    let mut header_buf = [0; 4];
    f.read_exact(&mut header_buf).ok()?;
    if header_buf != SNSS_HEADER {
        return None;
    }

    // Read version
    read_u32(&mut f)?;

    let mut tabs: HashMap<u32, Tab> = HashMap::new();
    let mut windows: HashMap<u32, Window> = HashMap::new();
    let mut groups: HashMap<String, TabGroup> = HashMap::new();

    // Read payload
    loop {
        let size = read_u16(&mut f);
        if size.is_none() {
            break;
        }
        let size = size.unwrap() as usize;
        let type_id = read_u8(&mut f)?;
        // Read payload
        let payload_buf = read_raw(&mut f, size - 1);
        if payload_buf.is_err() {
            debug!("Error reading payload {:?}", payload_buf.err());
            break;
        }
        let payload_buf = payload_buf.ok().unwrap();
        let mut payload = payload_buf.as_slice();

        debug!("Command: {:?}, Size: {}", type_id, size);

        let command = SSNSCommand::from_u8(type_id);
        if command.is_none() {
            debug!("Unknown command [{}], skipping", type_id);
            continue;
        }
        let command = command.unwrap();

        // Process command
        match command {
            SSNSCommand::UpdateTabNavigation => {
                read_u32(&mut payload); // Bypass size of payload

                let tab_id = read_u32(&mut payload)?;
                let history_index = read_u32(&mut payload)?;
                let url = read_string(&mut payload)?;
                let title = read_string_16(&mut payload)?;

                debug!("UpdateTabNavigation");
                debug!(
                    "Tab ID: {}, History Index: {}, URL: {}, Title: {}",
                    tab_id, history_index, url, title
                );

                let tab = get_tab(&mut tabs, tab_id);
                let history = tab.histories.iter_mut().find(|h| h.index == history_index);

                match history {
                    Some(h) => {
                        h.url = url;
                        h.title = title;
                    }
                    None => {
                        tab.histories.push(TabHistory {
                            index: history_index,
                            url,
                            title,
                        });
                    }
                }
            }
            SSNSCommand::SetSelectedTabInIndex => {
                debug!("SetSelectedTabInIndex");
                let window_id = read_u32(&mut payload)?;
                let active_tab_idx = read_u32(&mut payload)?;

                debug!("Window ID: {}, Active Tab Index: {}", window_id, active_tab_idx);

                let window = get_window(&mut windows, window_id);
                window.active_tab_index = active_tab_idx;
            }
            SSNSCommand::SetTabGroupMetadata2 => {
                debug!("SetTabGroupMetadata2");
                // Ignore size
                read_u32(&mut payload)?;

                let high = read_u64(&mut payload)?;
                let low = read_u64(&mut payload)?;
                let name = read_string_16(&mut payload)?;

                debug!("High: {}, Low: {}, Name: {}", high, low, name);

                let group = get_group(&mut groups, high, low);
                group.name = name;
            }
            SSNSCommand::SetTabGroup => {
                debug!("SetTabGroup");
                let tab_id = read_u32(&mut payload)?;
                read_u32(&mut payload)?; // Struct padding

                let high = read_u64(&mut payload)?;
                let low = read_u64(&mut payload)?;

                debug!("Tab ID: {}, High: {}, Low: {}", tab_id, high, low);
                let tab = get_tab(&mut tabs, tab_id);
                tab.group = groups.get(&format!("{}|{}", high, low)).cloned();
            }
            SSNSCommand::SetTabWindow => {
                debug!("SetTabWindow");
                let window_id = read_u32(&mut payload)?;
                let tab_id = read_u32(&mut payload)?;

                debug!("Window ID: {}, Tab ID: {}", window_id, tab_id);

                let tab = get_tab(&mut tabs, tab_id);
                tab.window_id = window_id;
            }
            SSNSCommand::WindowClosed => {
                debug!("WindowClosed");
                let window_id = read_u32(&mut payload)?;

                debug!("Window ID: {}", window_id);
                let window = get_window(&mut windows, window_id);
                window.deleted = true;
            }
            SSNSCommand::TabClosed => {
                debug!("TabClosed");
                let tab_id = read_u32(&mut payload)?;

                debug!("Tab ID: {}", tab_id);
                let tab = get_tab(&mut tabs, tab_id);
                tab.deleted = true;
            }
            SSNSCommand::SetTabIndexInWindow => {
                debug!("SetTabIndexInWindow");
                let tab_id = read_u32(&mut payload)?;
                let tab_index = read_u32(&mut payload)?;

                debug!("Tab ID: {}, Tab Index: {}", tab_id, tab_index);

                let tab = get_tab(&mut tabs, tab_id);
                tab.index = tab_index;
            }
            SSNSCommand::SetActiveWindow => {
                debug!("SetActiveWindow");
                let window_id = read_u32(&mut payload)?;

                debug!("Window ID: {}", window_id);
                let window = get_window(&mut windows, window_id);
                window.active = true;
            }
            SSNSCommand::SetSelectedNavigationIndex => {
                debug!("SetSelectedNavigationIndex");
                let tab_id = read_u32(&mut payload)?;
                let history_index = read_u32(&mut payload)?;

                debug!("Tab ID: {}, History Index: {}", tab_id, history_index);

                let tab = get_tab(&mut tabs, tab_id);
                tab.current_history_index = history_index;
            }
            SSNSCommand::LastActiveTime => {
                debug!("LastActiveTime");
                // TODO: Implement properly
            }
        }
    }

    debug!("Dcm wtf????");

    for (window_id, window) in &windows {
        debug!("Window: {}, {:?}", window_id, window);
    }
    let active_window = windows.iter_mut().find(|(_, w)| w.active && !w.deleted);
    if active_window.is_none() {
        return None;
    }
    debug!("Active window {:?}", active_window);
    let (active_window_id, active_window) = active_window.unwrap();

    for (_, tab) in tabs {
        if tab.deleted || tab.window_id != *active_window_id {
            continue;
        }
        active_window.tabs.push(tab);
    }
    active_window.tabs.sort_by(|a, b| a.index.cmp(&b.index));

    let active_tab = active_window
        .tabs
        .iter()
        .find(|t| t.index == active_window.active_tab_index);
    if active_tab.is_none() {
        return None;
    }
    let tab = active_tab.unwrap();
    let current_active_url = tab
        .histories
        .iter()
        .find(|h| h.index == tab.current_history_index)
        .map(|h| h.clone().url);

    current_active_url
}
