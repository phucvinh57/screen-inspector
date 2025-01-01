#![allow(dead_code)]
// This lib is used to read SNSS file of browsers
use std::{fs::File, io::Read};
use log::debug;

#[derive(Debug)]
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
    active_tab: u32,
    active: bool,
    deleted: bool,
}

const SNSS_HEADER: [u8; 4] = [0x53, 0x4E, 0x53, 0x53];

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

fn read_u8<R: Read>(mut f: R) -> Option<u8> {
    let mut buf = [0; 1];
    f.read_exact(&mut buf).ok().unwrap();
    return Some(buf[0]);
}

fn read_string<R: Read>(mut f: R) -> Option<String> {
    let size = read_u32(&mut f)?;
    let mut rsize = size;
    if rsize % 4 != 0 { // Chrome 32 bit align pickled data
        rsize += 4 - rsize % 4;
    }
    let mut buf = vec![0; rsize as usize];
    f.read_exact(&mut buf).ok()?;
    Some(String::from_utf8_lossy(&buf).to_string())
}

fn read_string_16<R: Read>(mut f: R) -> Option<String> {
    let size = read_u32(&mut f)?;
    let mut rsize = size * 2;
    if rsize % 4 != 0 { // Chrome 32 bit align pickled data
        rsize += 4 - rsize % 4;
    }
    let mut buf: Vec<u8> = vec![0; rsize as usize];
    f.read_exact(&mut buf).ok()?;

    let buf16 = buf.chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect::<Vec<u16>>();
    Some(String::from_utf16_lossy(&buf16).to_string())
}
fn read_raw<R: Read>(mut f: R, size: usize) -> Option<Vec<u8>> {
    let mut buf = vec![0; size];
    f.read_exact(&mut buf).ok()?;
    Some(buf)
}

// SNSS file format. No \n seperator
// "SNSS" (0x534E5353): 4 bytes
// <version>: Int32 (4 bytes), should be 1 or 3
// These are the commands that are stored in the SNSS file
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
    
    // Read version
    read_u32(&mut f)?;

    // Read payload
    loop {
        let size: usize = read_u16(&mut f)? as usize;
        let type_id = read_u8(&mut f)?;
        // Read payload
        let payload_buf = read_raw(&mut f, size - 1)?;
        let mut payload = payload_buf.as_slice();

        let command = SSNSCommand::from_u8(type_id);
        if command.is_none() {
            debug!("Unknown command [{}], skipping", type_id);
            continue;
        }
        let command = command.unwrap();

        // Process command
        match command {
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
            },
            SSNSCommand::SetTabIndexInWindow => {
            },
            
            SSNSCommand::SetSelectedNavigationIndex => {
            },
            SSNSCommand::SetSelectedTabInIndex => {
            },
            SSNSCommand::SetActiveWindow => {
            },
            SSNSCommand::LastActiveTime => {
            },
            SSNSCommand::SetTabGroup => {
            },
            SSNSCommand::SetTabGroupMetadata2 => {
            },
            SSNSCommand::TabClosed => {
            },
            SSNSCommand::WindowClosed => {
            },
        }
    }
}
