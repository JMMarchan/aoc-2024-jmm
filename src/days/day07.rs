use crate::{Solution, SolutionPair};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::fmt;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07.txt").expect("Day 7 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = total_calibration_result(&lines, false);
    let sol2 = total_calibration_result(&lines, true);
    (Solution::from(sol1), Solution::from(sol2))
}

// left to right, add or multiply, get total of all correct equations
fn total_calibration_result(input: &[&str], with_concat: bool) -> u64 {
    // brute force will certainly come back to bite me in part 2, there has to be some sort of logic
    input
        .iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let target = parts[0].trim().parse::<u64>().unwrap();
            let numbers: Vec<u64> = parts[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();

            if can_match_target(&numbers, target, 1, numbers[0], with_concat) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

// recursive
fn can_match_target(
    numbers: &[u64],
    target: u64,
    index: usize,
    current_value: u64,
    with_concat: bool,
) -> bool {
    // Base case
    if index == numbers.len() {
        return current_value == target;
    }

    let next_number = numbers[index];

    if can_match_target(
        numbers,
        target,
        index + 1,
        current_value + next_number,
        with_concat,
    ) {
        return true;
    }

    if current_value <= target / next_number
        && can_match_target(
            numbers,
            target,
            index + 1,
            current_value * next_number,
            with_concat,
        )
    {
        return true;
    }

    if with_concat {
        let concatenated_value = format!("{}{}", current_value, next_number)
            .parse::<u64>()
            .unwrap();
        if concatenated_value <= target
            && can_match_target(numbers, target, index + 1, concatenated_value, with_concat)
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#
    }

    #[test]
    fn test_total_calibration_result() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(total_calibration_result(&input, false), 3749);
    }
}
