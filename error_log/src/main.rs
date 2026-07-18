use chrono::{Timelike, Utc};

fn error_log<T: std::fmt::Display>(msg: &T) {
    let now = Utc::now();
    println!("{0}:{1}:{2} -- {msg}",
        now.hour(), now.minute(), now.second());
}

fn main() {
    let msg = "A message";
    let msg2 = 42;
    error_log(&msg);
    error_log(&msg2);
}
