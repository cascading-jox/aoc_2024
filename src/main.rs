use aoc_2024::solutions;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The day to run (1-25)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Run part two of the day's puzzle
    #[arg(short, long)]
    part_two: bool,
}

// use anyhow for simplified error handling (one error type instead of many), anyhow! macro,
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let result = match cli.day {
        1 => {
            if cli.part_two {
                solutions::day01::part_two()?
            } else {
                solutions::day01::part_one()?
            }
        }
        _ => {
            println!("Day {} not implemented yet!", cli.day);
            return Ok(());
        }
    };

    println!("Result: {}", result);
    Ok(())
}