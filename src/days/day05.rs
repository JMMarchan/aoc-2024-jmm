use crate::{Solution, SolutionPair};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use std::fmt;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day05.txt").expect("Day 5 input file should be present");
    let lines: Vec<&str> = input.lines().collect();
    let sol1 = sum_updates(&lines, true);
    let sol2 = sum_updates(&lines, false);
    (Solution::from(sol1), Solution::from(sol2))
}

// get middle value (assume all lists are odd length) of correctly sorted lists
fn sum_updates(input: &[&str], correctly_ordered: bool) -> u32 {
    let split_index = input.iter().position(|&line| line.is_empty()).unwrap();
    let mappings = &input[..split_index];
    let lists = &input[split_index + 1..];

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for mapping in mappings {
        let (a, b) = mapping
            .split('|')
            .map(|x| x.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        rules.entry(b).or_default().insert(a);
        rules.entry(a).or_default();
    }

    // println!("rules: {:?}", rules);

    lists
        .iter()
        .map(|list| {
            let nums: Vec<u32> = list
                .split(',')
                .map(|num| num.parse::<u32>().expect("Valid number"))
                .collect();

            let sorted_nums = sort_pages(&nums, &rules);
            if correctly_ordered == (*nums == sorted_nums) {
                // part 1 and part 2 are same, except checking opposite conditions
                sorted_nums[nums.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn sort_pages(nums: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let nums_set: HashSet<u32> = nums.iter().copied().collect();

    rules
        .iter()
        .filter_map(|(num, before)| {
            if nums_set.contains(num) {
                Some((*num, before.intersection(&nums_set).count()))
            } else {
                None
            }
        })
        .sorted_by_key(|(_, before_nums)| *before_nums)
        .map(|(num, _)| num)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
    }

    #[test]
    fn test_sum_correct_updates() {
        let input = test_input().lines().collect::<Vec<&str>>();
        assert_eq!(sum_correct_updates(&input), 143);
    }
}
