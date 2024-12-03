use anyhow::Result;
use regex::Regex;
use std::io::{self, BufRead};

pub fn solve_part_one(corrupt_memory: &str, debug: bool) -> i64 {
    // (\d+) creates a capture group per number
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(corrupt_memory)
        .map(|cap| {
            let x: i64 = cap[1].parse().unwrap();
            let y: i64 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

pub fn solve_part_two(corrupt_memory: &str, debug: bool) -> i64 {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    let mut enabled = true; // enabled by default
    let mut current_pos = 0;

    // create iterators for all do() and don't() positions
    let mut matches: Vec<_> = corrupt_memory
        .match_indices("do()")
        // combine two iterators into one
        .chain(corrupt_memory.match_indices("don't()"))
        .collect();
    // XXX important to sort them since matches are not in order
    matches.sort_by_key(|(pos, _)| *pos);

    for (pos, matched_text) in matches {
        if debug {
            println!("Found '{}' at position {}", matched_text, pos);
        }

        // process the section before the current do()
        if enabled {
            sum += process_section(&mul_re, &corrupt_memory[current_pos..pos]);
        }

        // set mode to enabled or disabled depending on the match
        if matched_text == "do()" {
            enabled = true;
            current_pos = pos + 4;
        } else if matched_text == "don't()" {
            enabled = false;
            current_pos = pos + 7;
        }
    }

    // process the final section
    if enabled {
        sum += process_section(&mul_re, &corrupt_memory[current_pos..]);
    }

    sum
}

fn process_section(mul_re: &Regex, section: &str) -> i64 {
    // same as before
    mul_re
        .captures_iter(section)
        .map(|cap| {
            let x: i64 = cap[1].parse().unwrap();
            let y: i64 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

fn parse_input(debug: bool) -> Result<String> {
    let input = std::fs::File::open("input/day03.txt")?;
    let reader = io::BufReader::new(input);

    let mut corrupt_memory = String::new();
    for line in reader.lines() {
        corrupt_memory.push_str(&line?);
    }

    if debug {
        println!("corrupt_memory: {:?}", corrupt_memory);
    }

    Ok(corrupt_memory)
}

pub fn part_one(debug: bool) -> Result<String> {
    let corrupt_memory = parse_input(debug)?;
    Ok(solve_part_one(&corrupt_memory, debug).to_string())
}

pub fn part_two(debug: bool) -> Result<String> {
    let corrupt_memory = parse_input(debug)?;
    Ok(solve_part_two(&corrupt_memory, debug).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(solve_part_one(string, true), 161);
    }

    #[test]
    fn test_part_two() {
        let string = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(solve_part_two(string, true), 48);
    }
}
