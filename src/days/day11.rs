use crate::{Solution, SolutionPair};
use hashbrown::{HashMap, HashSet};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day11.txt").expect("Day 11 input file should be present");
    let sol1 = count_stones(&input, 25);
    let sol2 = count_stones(&input, 75);

    (Solution::from(sol1), Solution::from(sol2))
}

fn count_stones(input: &str, splits: usize) -> u64 {
    use hashbrown::HashMap;

    let mut stones: HashMap<u64, usize> = HashMap::new();
    for num in input.split_whitespace() {
        let stone = num.parse::<u64>().unwrap();
        *stones.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..splits {
        // println!("stones: {:?}", stones);

        let mut next_stones: HashMap<u64, usize> = HashMap::new();

        for (&stone, &count) in stones.iter() {
            if stone == 0 {
                // Rule 1: 0 -> 1
                *next_stones.entry(1).or_insert(0) += count;
            } else {
                // Use logarithms to determine if the number of digits is even
                let num_digits = ((stone as f64).log10().floor() as usize) + 1;
                if num_digits % 2 == 0 {
                    // Rule 2: Even number of digits, split into two halves
                    let power = 10u64.pow((num_digits / 2) as u32);
                    let left = stone / power;
                    let right = stone % power;

                    *next_stones.entry(left).or_insert(0) += count;
                    *next_stones.entry(right).or_insert(0) += count;
                } else {
                    // Rule 3: Multiply by 2024
                    let new_stone = stone * 2024;
                    *next_stones.entry(new_stone).or_insert(0) += count;
                }
            }
        }

        stones = next_stones;
    }

    stones.iter().map(|(_, &count)| count as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_stones() {
        assert_eq!(count_stones("125 17", 6), 22);

        assert_eq!(count_stones("125 17", 25), 55312);
    }
}
