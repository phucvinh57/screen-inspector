use dotenv::dotenv;
use std::{thread::{self, sleep}, time::Duration};
use screen_inspector::window::get_current_window_information;
use screen_inspector::mouse::get_mouse_position;
fn main() {
    dotenv().ok();
    env_logger::init();

    let poll_time = (dotenv::var("METRICS_POLL_INTERVAL").unwrap_or("5000".to_owned()))
        .parse::<u64>()
        .unwrap();

    let afk_watch = thread::spawn(move || {
        loop {
            sleep(Duration::from_millis(poll_time));
            let pos = get_mouse_position();
            println!("Mouse position: {:?}", pos);
        }
    });

    let window_watcher = thread::spawn(move || {
        sleep(Duration::from_millis(poll_time));
        let info = get_current_window_information().unwrap();
        println!("Window information: {:?}", info);
    });

    window_watcher.join().unwrap();
    afk_watch.join().unwrap();
}
