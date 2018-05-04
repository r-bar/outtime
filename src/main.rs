extern crate clap;

use std::io::stdin;
use std::time::{Duration, SystemTime};
use clap::{App, Arg};

fn command_line_parser() -> App<'static, 'static> {
    return App::new("Output benchmarking program")
        .version("0.1")
        .about("Times the output of a program for rough benchmarking of simple scripts.")
        .arg(Arg::with_name("diff")
             .short("d")
             .long("diff")
             .help("Instead of displaying the total time elaped, shows the time elapsed since the previous line of output.")
        )
}

fn fmt_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let micros = duration.subsec_nanos();
    format!("{}.{:09}", secs, micros)
}

struct Timer {
    start_time: SystemTime,
    last_output_time: Option<SystemTime>,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: SystemTime::now(),
            last_output_time: None,
        }
    }

    fn diff_elasped(&mut self) -> Duration {
        let duration = match self.last_output_time {
            Some(last_time) => last_time.elapsed(),
            None => self.start_time.elapsed()
        };
        self.last_output_time = Some(SystemTime::now());
        duration.unwrap()
    }

    fn elapsed(&self) -> Duration {
        self.start_time.elapsed().unwrap()
    }
}

fn main() {
    let cli = command_line_parser();
    let matches = cli.get_matches();
    let input = stdin();
    let mut timer = Timer::new();
    loop {
        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => {
                let elapsed = timer.elapsed();
                println!("\nDone! Total elapsed: {}", fmt_duration(elapsed));
                break;
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
            Ok(_) => {
                let elapsed = if matches.is_present("diff") {
                    timer.diff_elasped()
                } else {
                    timer.elapsed()
                };
                println!("{} | {}", fmt_duration(elapsed), line.trim_right())
            }
        }
    }
}
