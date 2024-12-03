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

    /// Enable debug output
    #[arg(short, long)]
    debug: bool,
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
        2 => {
            if cli.part_two {
                solutions::day02::part_two(cli.debug)?
            } else {
                solutions::day02::part_one(cli.debug)?
            }
        }
        3 => {
            if cli.part_two {
                solutions::day03::part_two(cli.debug)?
            } else {
                solutions::day03::part_one(cli.debug)?
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
