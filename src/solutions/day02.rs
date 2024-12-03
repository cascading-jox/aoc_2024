use anyhow::Result;
use std::io::{self, BufRead};

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

        // check if original sequence is safe
        if is_safe_sequence(&report) {
            safe_reports += 1;
            continue;
        }

        // if not, check if removing a level makes the report safe
        /* 💭
           Removing a number has the same effect as using a flag but without the extra logic. We can also keep the original checks.
        */
        let mut is_safe = false;
        for i in 0..report.len() {
            let mut modified_report: Vec<i32> = report.clone();
            modified_report.remove(i);

            if is_safe_sequence(&modified_report) {
                is_safe = true;
                break;
            }
        }

        if debug {
            println!("Is safe with dampener: {}", is_safe);
            println!("-------------------\n");
        }

        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

fn is_safe_sequence(report: &[i32]) -> bool {
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

    (is_increasing || is_decreasing) && valid_differences
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

    Ok(solve_part_two(reports, debug).to_string())
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
