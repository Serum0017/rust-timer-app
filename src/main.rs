use notify_rust::Notification;

use chrono::{Timelike, Local};
use chrono::Datelike;
use chrono::Weekday;

use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    println!("Program running!");
    // how it works: checks once every minute if the time matches things in the match expression.
    // If it matches, we push a windows notification and remind the user!
    let interval = Duration::from_secs(60);
    let mut next_time = Instant::now() + interval;
    loop {
        let current = Local::now();

        // println!("it is {}, {}:{}", current.weekday(), current.hour(), current.minute());

        match (current.hour(), current.minute(), current.weekday()) {
            (21, 36, _) => display_notification("Get ready for bed!", ""),
            (13, 30, Weekday::Tue) => display_notification("Join physics class!", ""),
            (13, 30, Weekday::Thu) => display_notification("Join physics class!", ""),
            _ => ()
        }
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}

fn display_notification(title: &str, content: &str) {
    if content == "" {
        let _ = Notification::new()
            .summary(title)
            .timeout(0)
            .show();
    } else {
        let _ = Notification::new()
            .summary(title)
            .body(content)
            .timeout(0)
            .show();
    }
}