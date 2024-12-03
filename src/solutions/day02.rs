use anyhow::Result;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

pub fn solve_part_one(reports: Vec<Vec<i32>>, debug: bool) -> i32 {
    let mut safe_reports = 0;

    for report in reports {
        if debug {
            println!("Report: {:?}", report);
        }

        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut valid_differences = true;

        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];

            // Check for increasing sequence
            if report[i] <= report[i - 1] {
                is_increasing = false;
            }

            // Check for decreasing sequence
            if report[i] >= report[i - 1] {
                is_decreasing = false;
            }

            // Check adjacent differences
            if diff.abs() < 1 || diff.abs() > 3 {
                valid_differences = false;
            }
        }

        let is_safe = (is_increasing || is_decreasing) && valid_differences;

        if debug {
            println!("Is increasing: {}", is_increasing);
            println!("Is decreasing: {}", is_decreasing);
            println!("Valid differences: {}", valid_differences);
            println!("Is safe: {}", is_safe);
            println!("-------------------\n");
        }

        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn part_one(debug: bool) -> Result<String> {
    let input = std::fs::File::open("input/day02.txt")?;
    let reader = io::BufReader::new(input);
    let mut reports = vec![];

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        reports.push(report);
    }

    Ok(solve_part_one(reports, debug).to_string())
}

pub fn solve_part_two(reports: Vec<Vec<i32>>, debug: bool) -> i32 {
    let mut safe_reports = 0;

    for report in reports {
        if debug {
            println!("Report: {:?}", report);
        }

        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut valid_differences = true;
        let mut used_dampener = false;

        for i in 1..report.len() {
            let diff = report[i] - report[i - 1];

            // Check for increasing sequence
            if report[i] <= report[i - 1] {
                if !used_dampener {
                    used_dampener = true;
                    continue;
                }
                is_increasing = false;
            }

            // Check for decreasing sequence
            if report[i] >= report[i - 1] {
                if !used_dampener {
                    used_dampener = true;
                    continue;
                }
                is_decreasing = false;
            }

            // Check adjacent differences
            if diff.abs() < 1 || diff.abs() > 3 {
                if !used_dampener {
                    used_dampener = true;
                    continue;
                }
                valid_differences = false;
            }
        }

        let is_safe = (is_increasing || is_decreasing) && valid_differences;

        if debug {
            println!("Is increasing: {}", is_increasing);
            println!("Is decreasing: {}", is_decreasing);
            println!("Valid differences: {}", valid_differences);
            println!("Used dampener: {}", used_dampener);
            println!("Is safe: {}", is_safe);
            println!("-------------------\n");
        }

        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

pub fn part_two(debug: bool) -> Result<String> {
    let input = std::fs::File::open("input/day02.txt")?;
    let reader = io::BufReader::new(input);
    let mut reports = vec![];

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        reports.push(report);
    }

    Ok(solve_part_one(reports, debug).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(solve_part_one(reports, true), 2); // Enable debug for tests
    }

    #[test]
    fn test_part_two() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        assert_eq!(solve_part_two(reports, true), 4);
    }
}
