use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day04.txt").expect("Day 4 input file should be present");
    let lines: Vec<&str> = input.lines().collect();

    let sol1 = count_word_in_word_search(&lines);
    let sol2 = count_cross_word_in_grid(&lines);

    (Solution::from(sol1), Solution::from(sol2))
}

static WORD_TO_FIND: &str = "XMAS";

// XMAS can show up vertically, horizontally, or diagonally
fn count_word_in_word_search(input: &[&str]) -> u32 {
    let grid = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let rows = grid.len();
    let cols = grid[0].len();

    let word: Vec<char> = WORD_TO_FIND.chars().collect();

    let mut count = 0;

    // Check each starting point in the grid
    for row in 0..rows {
        for col in 0..cols {
            // Check in each direction
            for &(dy, dx) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ]
            .iter()
            {
                if matches_word(&grid, row as isize, col as isize, &word, dy, dx) {
                    count += 1;
                }
            }
        }
    }

    count
}

// Function to check if the word matches starting at (row, col) in the given direction
fn matches_word(
    grid: &[Vec<char>],
    start_row: isize,
    start_col: isize,
    word: &[char],
    dy: isize,
    dx: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for (i, &char_to_match) in word.iter().enumerate() {
        let new_row = start_row + dy * i as isize;
        let new_col = start_col + dx * i as isize;

        // Check if the new position is out of bounds
        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        // Check if the character matches
        if grid[new_row as usize][new_col as usize] != char_to_match {
            return false;
        }
    }

    true
}

fn count_cross_word_in_grid(input: &[&str]) -> u32 {
    let grid = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if grid[row][col] == 'A' {
                // Collect the four corners around the "A"
                let top_left = grid[row - 1][col - 1];
                let top_right = grid[row - 1][col + 1];
                let bottom_left = grid[row + 1][col - 1];
                let bottom_right = grid[row + 1][col + 1];

                // Check if these form any rotation of [M, M, S, S]
                let corners: Vec<char> = vec![top_left, top_right, bottom_left, bottom_right];

                if corners == "MMSS".chars().collect::<Vec<char>>()
                    || corners == "MSMS".chars().collect::<Vec<char>>()
                    || corners == "SSMM".chars().collect::<Vec<char>>()
                    || corners == "SMSM".chars().collect::<Vec<char>>()
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#
    }

    #[test]
    fn test_count_word_in_word_search() {
        let lines: Vec<&str> = test_input().lines().collect();
        assert_eq!(count_word_in_word_search(&lines), 18);
    }

    #[test]
    fn test_count_cross_word_in_grid() {
        let grid: Vec<&str> = test_input().lines().collect();
        assert_eq!(count_cross_word_in_grid(&grid), 9);
    }
}
