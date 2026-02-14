mod components;
mod config;
pub mod utils;

use tokio::sync::watch;
use tokio::time::{Duration, interval};

fn setup(tx: watch::Sender<String>) {
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(1));
        let mut seconds_passed: u64 = 0;

        let mut cache: Vec<String> = vec!["".to_string(); config::COMPONENTS.len()];
        let mut buffer = String::with_capacity(256);

        loop {
            ticker.tick().await;
            buffer.clear();

            for (i, comp) in config::COMPONENTS.iter().enumerate() {
                if seconds_passed.is_multiple_of(comp.interval_s) {
                    cache[i] = (comp.func)(comp.arg.unwrap());
                }

                let formatted = comp.fmt.replace("%s", &cache[i]);
                buffer.push_str(&formatted);
            }

            let _ = tx.send(buffer.clone());
            seconds_passed = seconds_passed.wrapping_add(1);
        }
    });
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = watch::channel("Loading...".to_string());

    setup(tx);

    loop {
        if rx.changed().await.is_ok() {
            let current_status = rx.borrow();
            println!("{}", *current_status);
        }
    }
}
