use clap::Parser;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use indicatif::{ProgressBar, ProgressStyle};
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// Number of minutes to time
    #[arg(short, long, default_value_t=50)]
    minutes : u64,

    #[arg(short, long)]
    start : bool

}

fn main() {
    let start_time = SystemTime::now();
    let args = Cli::parse();
    let minutes = args.minutes;
    let mut done = false;
    let start = args.start;

    if start == true{
        let pb = ProgressBar::new(minutes);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} {bar:40.cyan/red} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

        println!("Thinking!");
        while !done {
            let elapsed_time = SystemTime::now().duration_since(start_time).expect("failed to calculate elapsed time") ;
            let elapsed_minutes = elapsed_time.as_secs() / 60 ;
            let time_remaining = minutes - elapsed_minutes;
            if time_remaining == 0{
                done = true
            }
            pb.set_position( elapsed_minutes);
            sleep(Duration::from_millis(50));
        
       
        }
        pb.finish_with_message("Done!");

    }
}


// fn main() {
//     let args = std::env::args()
//     let time = args.nth(1).expect("no time given")
// }
