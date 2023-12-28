use screen_inspector::mouse::get_mouse_position;
use screen_inspector::window::{get_current_window_information, WindowInformation};
use std::{
    env,
    thread::{self, sleep},
    time::Duration,
};
fn main() {
    let poll_time = env::var("METRICS_POLL_INTERVAL")
        .unwrap_or("5000".to_owned())
        .parse::<u64>()
        .unwrap();

    let afk_watch = thread::spawn(move || loop {
        sleep(Duration::from_millis(poll_time));
        let pos = get_mouse_position();
        println!("Mouse position: {:?}", pos);
    });

    let window_watcher = thread::spawn(move || loop {
        sleep(Duration::from_millis(poll_time));
        let window_info: WindowInformation = get_current_window_information().unwrap();
        print!("Window information: {:?}", window_info);
    });

    window_watcher.join().unwrap();
    afk_watch.join().unwrap();
}
