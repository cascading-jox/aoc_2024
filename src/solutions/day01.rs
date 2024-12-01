use anyhow::Result;
use std::fs;
use std::io::{self, BufRead};

pub fn solve_part_one(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();

    let mut total_distance = 0;
    for n in 0..left_list.len() {
        let diff = left_list[n] - right_list[n];
        total_distance += diff.abs();
    }

    total_distance
}

pub fn part_one() -> Result<String> {
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

    Ok(solve_part_one(left_list, right_list).to_string())
}

pub fn part_two() -> Result<String> {
    let input = fs::read_to_string("input/day01.txt")?;
    // Implement solution for part two
    Ok("Not implemented yet".to_string())
}

// -------------- Tests ----------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(solve_part_one(left, right), 11);
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two().unwrap();
    //     assert_eq!(result, "Not implemented yet");
    // }
}
