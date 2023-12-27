use dotenv::dotenv;
use std::thread;
use tpulse::watcher::watch_window;
fn main() {
    dotenv().ok();
    env_logger::init();

    let poll_time = (dotenv::var("METRICS_POLL_INTERVAL").unwrap_or("5000".to_owned()))
        .parse::<u64>()
        .unwrap();

    // let afk_settings = AFKSettings::new(5000, poll_time);
    // let afk_watcher = AFKWatcher::new(&afk_settings);

    // let afk_watch = thread::spawn(move || {
    //     afk_watcher.run();
    // });

    let window_watcher = thread::spawn(move || {
        watch_window(poll_time);
    });

    window_watcher.join().unwrap();
    // afk_watch.join().unwrap();
}
