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

pub fn solve_part_two(left_list: Vec<i32>, right_list: Vec<i32>) -> i32 {
    let mut num_counter = HashMap::new();
    let mut similarity_score = 0;

    for i in 0..left_list.len() {
        if !num_counter.contains_key(&left_list[i]) {
            for j in 0..left_list.len() {
                if left_list[i] == right_list[j] {
                    num_counter
                        .entry(left_list[i])
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
            }
        }
    }

    for n in 0..left_list.len() {
        similarity_score += left_list[n] * num_counter.get(&left_list[n]).unwrap_or(&0);
    }

    similarity_score
}

pub fn part_two() -> Result<String> {
    let input = std::fs::File::open("input/day01.txt")?;
    let reader = io::BufReader::new(input);

    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in reader.lines() {
        // handle line reading in case of failure
        // shorter way of match on line (line is Result)
        let line = line?;
        let mut iter = line.split_ascii_whitespace();
        left_list.push(iter.next().unwrap().parse::<i32>().unwrap());
        right_list.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    Ok(solve_part_two(left_list, right_list).to_string())
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
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(solve_part_two(left, right), 31);
    }
}
