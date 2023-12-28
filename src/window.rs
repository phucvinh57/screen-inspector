use anyhow::{anyhow, Ok, Result};
use log::info;
use regex::Regex;
use serde::Serialize;
use std::{process::Command, thread::sleep, time::Duration};
use std::fs;

#[derive(Debug, Serialize)]
pub struct WindowInformation {
    title: Option<String>,
    class: Option<Vec<String>>,
    exec_path: Option<String>,
}


pub fn get_current_window_information() -> Result<WindowInformation> {
    let window_raw_id = get_window_id().unwrap();
    let window_info = get_window_information_by_id(window_raw_id)?;
    Ok(window_info)
}

#[cfg(target_os = "linux")]
fn get_window_id() -> Result<i64> {
    let bin = "xprop";
    let args = ["-root", "_NET_ACTIVE_WINDOW"];
    let window_id_regex = Regex::new(r"0x[a-fA-F0-9]+").unwrap();
    let window_raw_id = Command::new(bin)
        .args(&args)
        .output()
        .expect("Failed to execute command");

    if !window_raw_id.status.success() {
        let stderr = String::from_utf8_lossy(&window_raw_id.stderr);
        return Err(anyhow!("Get window ID error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&window_raw_id.stdout);
    if let Some(captured) = window_id_regex.find(&stdout) {
        let window_id_hex = captured.as_str().trim_start_matches("0x");
        let window_id = i64::from_str_radix(window_id_hex, 16).unwrap();
        return Ok(window_id);
    }
    Err(anyhow!("No window ID found in the input string."))
}

#[cfg(target_os = "linux")]
fn get_window_information(window_id: i64) -> Result<WindowInformation> {
    let bin = "xprop";
    let window_raw_infor = Command::new(bin)
        .env("LC_ALL", "C.utf8")
        .arg("-id")
        .arg(window_id.to_string())
        .arg("-notype")
        .arg("WM_NAME")
        .arg("WM_CLASS")
        .arg("_NET_WM_NAME")
        .arg("_NET_WM_PID")
        .output()
        .expect("Failed to execute command");

    if !window_raw_infor.status.success() {
        let stderr = String::from_utf8_lossy(&window_raw_infor.stderr);
        return Err(anyhow!("Get window information error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&window_raw_infor.stdout);
    let mut window_info = WindowInformation {
        title: None,
        class: None,
        exec_path: None,
    };

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
        if parts.len() < 2 {
            continue;
        }

        match parts[0] {
            "WM_NAME" => window_info.title = Some(parts[1].trim_matches('"').to_string()),
            "WM_CLASS" => {
                let class = parts[1].split(',').map(|s| s.trim().trim_matches('"'));
                window_info.class = Some(class.map(|s| s.to_string()).collect());
            }
            "_NET_WM_PID" => {
                let pid = parts[1].trim().parse::<i32>().unwrap();
                if let std::result::Result::Ok(exec_path) =
                    fs::read_link(format!("/proc/{}/exe", pid))
                {
                    let path_str = exec_path.as_path().display().to_string();
                    window_info.exec_path = Some(path_str);
                }
            }
            "_NET_WM_NAME" => {
                if window_info.title.is_none() {
                    window_info.title = Some(parts[1].trim_matches('"').to_string());
                }
            }
            _ => {}
        }
    }

    Ok(window_info)
}

#[cfg(target_os = "windows")]
fn get_window_information_by_id(window_id: i64) -> Result<WindowInformation> {
    unimplemented!("Windows is not supported yet.")
}

#[cfg(target_os = "windows")]
fn get_window_id() -> Result<i64> {
    unimplemented!("Windows is not supported yet.")
}

#[cfg(target_os = "macos")]
fn get_window_information(window_id: i64) -> Result<WindowInformation> {
    unimplemented!("MacOS is not supported yet.")
}

#[cfg(target_os = "macos")]
fn get_window_id() -> Result<i64> {
    unimplemented!("MacOS is not supported yet.")
}
