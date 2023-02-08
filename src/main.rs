use clap::Parser;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use indicatif::{ProgressBar, ProgressStyle};



static START_TEXT : &str = "\x1b[91mE
________  ___  ___  ________       ___    ___      ___       __   ________  ________  ___  __    ___  ________   ________     
|\\   __  \\|\\  \\|\\  \\|\\   ____\\     |\\  \\  /  /|    |\\  \\     |\\  \\|\\   __  \\|\\   __  \\|\\  \\|\\  \\ |\\  \\|\\   ___  \\|\\   ____\\    
\\ \\  \\|\\ /\\ \\  \\\\\\  \\ \\  \\___|_    \\ \\  \\/  / /    \\ \\  \\    \\ \\  \\ \\  \\|\\  \\ \\  \\|\\  \\ \\  \\/  /|\\ \\  \\ \\  \\\\ \\  \\ \\  \\___|    
 \\ \\   __  \\ \\  \\\\\\  \\ \\_____  \\    \\ \\    / /      \\ \\  \\  __\\ \\  \\ \\  \\\\\\  \\ \\   _  _\\ \\   ___  \\ \\  \\ \\  \\\\ \\  \\ \\  \\  ___  
  \\ \\  \\|\\  \\ \\  \\\\\\  \\|____|\\  \\    \\/  /  /        \\ \\  \\|\\__\\_\\  \\ \\  \\\\\\  \\ \\  \\\\  \\\\ \\  \\\\ \\  \\ \\  \\ \\  \\\\ \\  \\ \\  \\|\\  \\ 
   \\ \\_______\\ \\_______\\____\\_\\  \\ __/  / /           \\ \\____________\\ \\_______\\ \\__\\\\ _\\\\ \\__\\\\ \\__\\ \\__\\ \\__\\\\ \\__\\ \\_______\\
    \\|_______|\\|_______|\\_________\\\\___/ /             \\|____________|\\|_______|\\|__|\\|__|\\|__| \\|__|\\|__|\\|__| \\|__|\\|_______|
                       \\|_________\\|___|/                                                                                      
\x1b[0m
";
static END_TEXT : &str= "\x1b[92mE
________  _______   ________  ________      ___    ___      _________  ________          _________  ________  ___       ___  __       
|\\   __  \\|\\  ___ \\ |\\   __  \\|\\   ___ \\    |\\  \\  /  /|    |\\___   ___\\\\   __  \\        |\\___   ___\\\\   __  \\|\\  \\     |\\  \\|\\  \\     
\\ \\  \\|\\  \\ \\   __/|\\ \\  \\|\\  \\ \\  \\_|\\ \\   \\ \\  \\/  / /    \\|___ \\  \\_\\ \\  \\|\\  \\       \\|___ \\  \\_\\ \\  \\|\\  \\ \\  \\    \\ \\  \\/  /|_   
 \\ \\   _  _\\ \\  \\_|/_\\ \\   __  \\ \\  \\ \\\\ \\   \\ \\    / /          \\ \\  \\ \\ \\  \\\\\\  \\           \\ \\  \\ \\ \\   __  \\ \\  \\    \\ \\   ___  \\  
  \\ \\  \\\\  \\\\ \\  \\_|\\ \\ \\  \\ \\  \\ \\  \\_\\\\ \\   \\/  /  /            \\ \\  \\ \\ \\  \\\\\\  \\           \\ \\  \\ \\ \\  \\ \\  \\ \\  \\____\\ \\  \\\\ \\  \\ 
   \\ \\__\\\\ _\\\\ \\_______\\ \\__\\ \\__\\ \\_______\\__/  / /               \\ \\__\\ \\ \\_______\\           \\ \\__\\ \\ \\__\\ \\__\\ \\_______\\ \\__\\\\ \\__\\
    \\|__|\\|__|\\|_______|\\|__|\\|__|\\|_______|\\___/ /                 \\|__|  \\|_______|            \\|__|  \\|__|\\|__|\\|_______|\\|__| \\|__|
                                           \\|___|/                                                                                     
\x1b[0m                                                                                                                                                                                                                                                                                                                                                           
";


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
        pb.set_style(ProgressStyle::with_template("{spinner:.blue} {bar:140.cyan/red} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

        println!("{}", START_TEXT);
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
        pb.finish_with_message(END_TEXT);

    }
}


// fn main() {
//     let args = std::env::args()
//     let time = args.nth(1).expect("no time given")
// }
