// This lib is used to read SNSS file of browsers
use std::{fs::File, io::Read};

use log::debug;

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

struct Tab {
    active: bool,
    url: String,
    title: String,
    deleted: bool,
}

struct Window {
    tabs: Vec<Tab>,
    active: bool,
    deleted: bool,
}

const SNSS_HEADER: [u8; 4] = [0x53, 0x4E, 0x53, 0x53];

fn read_u16(mut f: &[u8]) -> Option<u16> {
    let mut buf = [0; 2];
    f.read_exact(&mut buf).ok()?;
    Some(u16::from_le_bytes(buf))
}
fn read_u32(mut f: &[u8]) -> Option<u32> {
    let mut buf = [0; 4];
    f.read_exact(&mut buf).ok()?;
    Some(u32::from_le_bytes(buf))
}
fn read_u8(mut f: &[u8]) -> Option<u8> {
    let mut buf = [0; 1];
    f.read_exact(&mut buf).ok()?;
    Some(u8::from_le_bytes(buf))
}
fn read_string(mut f: &[u8]) -> Option<String> {
    let size = read_u32(f)?;
    let mut rsize = size;
    if rsize % 4 != 0 { // Chrome 32 bit align pickled data
        rsize += 4 - rsize % 4;
    }
    let mut buf = vec![0; rsize as usize];
    f.read_exact(&mut buf).ok()?;
    Some(String::from_utf8_lossy(&buf).to_string())
}
fn read_string_16(mut f: &[u8]) -> Option<String> {
    let size = read_u32(f)?;
    let mut rsize = size * 2;
    if rsize % 4 != 0 { // Chrome 32 bit align pickled data
        rsize += 4 - rsize % 4;
    }
    let mut buf: Vec<u8> = vec![0; rsize as usize];
    f.read_exact(&mut buf).ok()?;

    let buf16 = buf.chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect::<Vec<u16>>();
    Some(String::from_utf16_lossy(&buf16).to_string())
}

// SNSS file format. No \n seperator
// "SNSS" (0x534E5353): 4 bytes
// <version>: Int32, should be 1 or 3
// <int16(size)><int8(type id)><payload(size - 1 bytes)>
// When user do an action, browser will append a "command" to SNSS file
pub fn read_snss_file(path: String) -> Option<()> {
    let mut f = File::open(path).ok()?;

    // Read header
    let mut header_buf = [0; 4];
    f.read_exact(&mut header_buf).ok()?;
    if header_buf != SNSS_HEADER {
        return None;
    }
    let header_str = String::from_utf8_lossy(&header_buf).to_string();
    debug!("Header: {}", header_str);

    // Read version
    let mut version_buf = [0; 4];
    f.read_exact(&mut version_buf).unwrap();
    let version = i32::from_le_bytes(version_buf);
    if version != 1 && version != 3 {
        return None;
    }
    debug!("Version: {}", version);

    // Read payload
    loop {
        // Read size
        let mut size_buf = [0; 2];
        if f.read_exact(&mut size_buf).is_err() {
            break;
        }
        let size = u16::from_le_bytes(size_buf) as usize;

        // Read type id
        let mut type_id_buf = [0; 1];
        if f.read_exact(&mut type_id_buf).is_err() {
            break;
        }
        let type_id = type_id_buf[0];

        // Read payload
        let mut payload_buf = vec![0; size - 1];
        if f.read_exact(&mut payload_buf).is_err() {
            break;
        }
        let payload = payload_buf.clone();
        let mut payload = payload.as_slice();

        // Convert type id to enum
        let type_id = SSNSCommand::from_u8(type_id);
        if type_id.is_none() {
            continue;
        }
        let type_id = type_id.unwrap();

        // Process command
        match type_id {
            SSNSCommand::UpdateTabNavigation => {
                debug!("UpdateTabNavigation");
                read_u32(&mut payload); // Bypass size of payload

                let tab_id = read_u32(&mut payload)?;
                let history_idx = read_u32(&mut payload)?;
                let url = read_string(&mut payload)?;
                let title = read_string_16(&mut payload)?;

                debug!("Tab ID: {}", tab_id);
                debug!("History Index: {}", history_idx);
                debug!("URL: {}", url);
                debug!("Title: {}", title);
            },
            SSNSCommand::SetTabWindow => {
                debug!("SetTabWindow");
            },
            SSNSCommand::SetTabIndexInWindow => {
                debug!("SetTabIndexInWindow");
            },
            
            SSNSCommand::SetSelectedNavigationIndex => {
                debug!("SetSelectedNavigationIndex");
            },
            SSNSCommand::SetSelectedTabInIndex => {
                debug!("SetSelectedTabInIndex");
            },
            SSNSCommand::SetActiveWindow => {
                debug!("SetActiveWindow");
            },
            SSNSCommand::LastActiveTime => {
                debug!("LastActiveTime");
            },
            SSNSCommand::SetTabGroup => {
                debug!("SetTabGroup");
            },
            SSNSCommand::SetTabGroupMetadata2 => {
                debug!("SetTabGroupMetadata2");
            },
            SSNSCommand::TabClosed => {
                debug!("TabClosed");
            },
            SSNSCommand::WindowClosed => {
                debug!("WindowClosed");
            },
        }
    }

    Some(())
}
