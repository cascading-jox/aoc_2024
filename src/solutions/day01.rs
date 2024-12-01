use anyhow::Result;
use std::fs;

pub fn part_one() -> Result<String> {
    let input = fs::read_to_string("input/day01.txt")?;
    // Implement solution for part one
    Ok("Not implemented yet".to_string())
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
        let result = part_one().unwrap();
        assert_eq!(result, "Not implemented yet");
    }

    #[test]
    fn test_part_two() {
        let result = part_two().unwrap();
        assert_eq!(result, "Not implemented yet");
    }
}
