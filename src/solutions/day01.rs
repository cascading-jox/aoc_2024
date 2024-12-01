use anyhow::Result;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

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

    #[test]
    fn test_part_two() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(solve_part_two(left, right), 31);
    }
}
