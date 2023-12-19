use clap::Parser;

pub mod day1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let mut run_all = false;
    let mut day_to_run = 0;
    if let Some(day_number) = args.day {
        println!("running day {day_number}");
        day_to_run = 0;
    } else {
        run_all = true;
        println!("running all days");
    }

    if run_all || day_to_run == 1 {
        day1::run_day();
    }
}
