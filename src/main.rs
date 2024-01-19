use notify_rust::Notification;

use chrono::{Timelike, Local};
use chrono::Datelike;
use chrono::Weekday;

use std::thread::sleep;
use std::time::{Duration, Instant};

// use std::fs::File;
use std::io::BufReader;
use std::fs::File;

use rodio::OutputStream;

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
            (21, 55, _) => display_notification("Get ready for bed!", ""),// 9:55 pm
            (13, 25, Weekday::Tue | Weekday::Thu) => {// 1:25 pm
                display_notification("Join physics class!", "");
                play_sound("assets/loonboon.ogg");
            },
            (9, 40, Weekday::Tue | Weekday::Thu) => display_notification("Join taa!", ""),// 9:40 am
            (13, 25, Weekday::Fri) => display_notification("Join Robotics!", ""),// 1:25 pm
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

fn play_sound(src: &str){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(src).unwrap());
    let loonboon = stream_handle.play_once(file).unwrap();

    sleep(Duration::from_secs(12));

    drop(loonboon);
}