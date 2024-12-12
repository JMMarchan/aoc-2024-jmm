use crate::{Solution, SolutionPair};
use grid::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day10.txt").expect("Day 10 input file should be present");
    let sol1: usize = count_trailhead_scores(&input);
    let sol2: usize = get_trails(&input).len();

    (Solution::from(sol1), Solution::from(sol2))
}

type Position = (usize, usize);
type Trail = Vec<Position>;

fn parse_input_grid(input: &str) -> Grid<char> {
    let grid_vec: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    let grid_cols = input.find('\n').unwrap_or(input.len());
    Grid::from_vec(grid_vec.clone(), grid_cols)
}

fn count_trailhead_scores(input: &str) -> usize {
    get_trails(&input)
        .iter()
        .filter_map(|trail| Some((*trail.first()?, *trail.last()?)))
        .collect::<HashSet<(Position, Position)>>()
        .len()
}

fn get_trails(input: &str) -> HashSet<Trail> {
    let grid = parse_input_grid(input);
    let mut trails: HashSet<Trail> = HashSet::new();

    let start_positions: Vec<Position> = grid
        .indexed_iter()
        .filter_map(|((i, j), &char)| if char == '0' { Some((i, j)) } else { None })
        .collect();

    for &start in &start_positions {
        let mut current_trail = vec![start];
        let mut visited = HashSet::new();
        visited.insert(start);
        find_trails(&grid, start, &mut current_trail, &mut trails, &mut visited);
    }

    trails
}

fn find_trails(
    grid: &Grid<char>,
    position: Position,
    current_trail: &mut Trail,
    trails: &mut HashSet<Trail>,
    visited: &mut HashSet<Position>,
) {
    if current_trail.len() == 10 {
        if grid.get(position.0, position.1) == Some(&'9') {
            trails.insert(current_trail.clone());
        }
        return;
    }

    let current_height = match grid
        .get(position.0, position.1)
        .and_then(|c| c.to_digit(10))
    {
        Some(height) => height,
        None => return,
    };

    let neighbors = vec![
        (position.0.wrapping_sub(1), position.1), // Up
        (position.0 + 1, position.1),             // Down
        (position.0, position.1.wrapping_sub(1)), // Left
        (position.0, position.1 + 1),             // Right
    ];

    for neighbor in neighbors {
        if visited.contains(&neighbor) {
            continue;
        }

        if let Some(neighbor_height) = grid
            .get(neighbor.0, neighbor.1)
            .and_then(|c| c.to_digit(10))
        {
            if neighbor_height == current_height + 1 {
                visited.insert(neighbor);
                current_trail.push(neighbor);

                find_trails(grid, neighbor, current_trail, trails, visited);

                current_trail.pop();
                visited.remove(&neighbor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input_small() -> &'static str {
        r#"0123
5434
6789
0198
6789
5434
0123
"#
    }

    #[test]
    fn test_count_trails_small() {
        let input = test_input_small();
        assert_eq!(count_trailhead_scores(&input), 4);
    }

    fn test_input() -> &'static str {
        r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#
    }

    #[test]
    fn test_count_trails() {
        let input = test_input();
        assert_eq!(count_trailhead_scores(&input), 36);
    }
}
