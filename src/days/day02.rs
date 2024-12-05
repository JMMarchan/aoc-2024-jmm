use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day02.txt").expect("Day 2 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = count_safe_reports(&lines);
    let sol2 = count_safe_reports_with_dampener(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

fn count_safe_reports(input: &[&str]) -> u32 {
    input
        .iter()
        .filter(|line: &&&str| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            // println!("{} safe: {}", line, is_safe(&numbers));
            is_safe(&numbers)
        })
        .count() as u32
}

fn is_safe(numbers: &[i32]) -> bool {
    let differences: Vec<i32> = numbers.windows(2).map(|pair| &pair[1] - &pair[0]).collect();

    differences.iter().all(|&diff| (1..=3).contains(&diff)) // All increasing
        || differences.iter().all(|&diff| (-3..=-1).contains(&diff))
}

fn count_safe_reports_with_dampener(input: &[&str]) -> u32 {
    input
        .iter()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            // Base case: If the sequence is safe, return true
            if is_safe(&numbers) {
                return true;
            }

            // Try removing one number at a time and check for safety
            for i in 0..numbers.len() {
                let mut reduced_numbers = numbers.clone();
                reduced_numbers.remove(i);
                if is_safe(&reduced_numbers) {
                    return true;
                }
            }

            false
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#
    }

    #[test]
    fn test_count_safe_reports() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(count_safe_reports(&input), 2);
    }

    #[test]
    fn test_count_safe_reports_with_dampener() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(count_safe_reports_with_dampener(&input), 4);
    }
}
