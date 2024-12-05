use crate::{Solution, SolutionPair};
use rayon::prelude::*;
use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day01.txt").expect("Day 1 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = total_distance(&lines);
    let sol2 = total_similarity_score(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

fn total_distance(input: &[&str]) -> u32 {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    input.iter().for_each(|line| {
        if let Some((left, right)) = line.split_once(' ') {
            left_list.push(left.trim().parse().expect("Wrong left"));
            right_list.push(right.trim().parse().expect("Wrong right"));
        }
    });

    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(left, right)| (*left as i32 - *right as i32).abs() as u32)
        .sum()
}

fn total_similarity_score(input: &[&str]) -> u32 {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    input.iter().for_each(|line| {
        if let Some((left, right)) = line.split_once(' ') {
            left_list.push(left.trim().parse().expect("Wrong left"));
            right_list.push(right.trim().parse().expect("Wrong right"));
        }
    });

    use std::collections::HashMap;
    let mut right_count: HashMap<u32, u32> = HashMap::new();
    for &num in &right_list {
        *right_count.entry(num).or_insert(0) += 1;
    }

    left_list
        .iter()
        .map(|&num| num * right_count.get(&num).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"3   4
4   3
2   5
1   3
3   9
3   3
"#
    }

    #[test]
    fn test_total_distance() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(total_distance(&input), 11);
    }

    #[test]
    fn test_total_similarity_score() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(total_similarity_score(&input), 31);
    }
}
