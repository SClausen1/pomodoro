mod consts;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use pomodoro::Timer;
use std::thread::sleep;
use std::time::Duration;

/// Easy CLI Pomodoro Timer.
#[derive(Parser)]
struct Cli {
    /// Number of minutes to time
    #[arg(short, long, default_value_t = 50)]
    minutes: u64,

    /// start the timer
    #[arg(short, long)]
    start: bool,
}

fn main() {
    let args = Cli::parse();
    let minutes = args.minutes;
    let timer = Timer::new(minutes);
    let mut done = false;
    let start = args.start;

    if start == true {
        let pb = ProgressBar::new(minutes);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.blue} {bar:70.cyan/red} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );

        println!("{}", consts::START_TEXT);
        while !done {
            let elapsed_minutes = timer.get_elapsed_minutes();
            if minutes == elapsed_minutes {
                done = true
            }
            pb.set_position(elapsed_minutes);
            sleep(Duration::from_millis(50));
        }
        pb.finish_with_message(consts::END_TEXT);
    }
}
