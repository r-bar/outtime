extern crate clap;

use std::io::stdin;
use std::time::SystemTime;

fn done(start_time: &SystemTime) {
    let elapsed = start_time.elapsed().unwrap();
    let secs = elapsed.as_secs();
    let micros = elapsed.subsec_nanos();
    println!("\nDone! Total elapsed: {}.{:09}", secs, micros);
}

fn output_time(start_time: &SystemTime, line: &str) {
    let elapsed = start_time.elapsed().unwrap();
    let secs = elapsed.as_secs();
    let micros = elapsed.subsec_nanos();
    println!("{}.{:09} | {}", secs, micros, line.trim_right());
}

fn main() {
    let mut more: bool = true;
    let input = stdin();
    let start_time = SystemTime::now();
    while more {
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => {
                more = false;
                done(&start_time);
            }
            Err(_) => {
                more = false;
                done(&start_time);
            }
            Ok(_) => {
                output_time(&start_time, &line);
            }
        }
    }
}
