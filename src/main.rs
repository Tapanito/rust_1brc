use clap::{Parser, ValueEnum};
use std::io::BufReader;
use std::time::{Duration, Instant};
pub mod single_thread;
pub mod storage;

#[derive(Parser)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,

    #[arg(long, short, default_value_t = String::from("data/small.csv"))]
    file: String,

    #[arg(long, short, default_value_t = 1)]
    iterations: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    SingleThread,
    MultiThread,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Args::parse();

    let mut durations: Vec<Duration> = Vec::new();
    for i in 0..cli.iterations {
        println!("Iteration: {}", i);
        let start = Instant::now();
        let file = std::fs::File::open(cli.file.clone())?;
        let mut reader = BufReader::with_capacity(1024 * 1024, file);
        single_thread::run(&mut reader);

        let end = start.elapsed();

        durations.push(end);
    }

    let mean =
        (durations.iter().map(|d| d.as_millis()).sum::<u128>() as f64) / durations.len() as f64;
    println!("Average duration: {}", mean);
    Ok(())
}
