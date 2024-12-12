use crate::{Solution, SolutionPair};
use chrono::format::parse;
use grid::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::fmt;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day12.txt").expect("Day 12 input file should be present");
    let sol1: usize = total_price_fencing(&input);
    let sol2: usize = total_price_fencing_sides(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

type Position = (usize, usize);

fn parse_input_grid(input: &str) -> Grid<char> {
    let grid_vec: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    let grid_cols = input.find('\n').unwrap_or(input.len());
    Grid::from_vec(grid_vec.clone(), grid_cols)
}

fn bfs(
    position: Position,
    visited: &mut HashSet<Position>,
    grid: &Grid<char>,
    plant_type: char,
) -> (HashSet<Position>, usize) {
    let mut queue = VecDeque::new();

    let mut area = HashSet::new();
    let mut perimeter = 0;

    queue.push_back(position);

    while let Some(current) = queue.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        area.insert(current);

        let neighbors = get_valid_neighbors(current, plant_type, &grid);

        perimeter += 4 - neighbors.len();

        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    // println!(
    //     "plant: {}, area: {}, perimeter: {}",
    //     plant_type,
    //     area.len(),
    //     perimeter
    // );

    (area, perimeter)
}

fn get_valid_neighbors(position: Position, plant_type: char, grid: &Grid<char>) -> Vec<Position> {
    [(1, 0), (0, 1), (0, -1), (-1, 0)]
        .iter()
        .filter_map(|&(dx, dy)| {
            let (x, y) = (position.0 as isize + dx, position.1 as isize + dy);

            if x >= 0 && y >= 0 {
                let (x, y) = (x as usize, y as usize);
                if let Some(&char) = grid.get(x, y) {
                    if char == plant_type {
                        return Some((x, y));
                    }
                }
            }
            None
        })
        .collect()
}

fn total_price_fencing(input: &str) -> usize {
    let grid: Grid<char> = parse_input_grid(input);
    let mut visited: HashSet<Position> = HashSet::new();

    grid.indexed_iter().fold(0, |acc, (position, &plant_type)| {
        if visited.contains(&position) {
            acc
        } else {
            let (area, perimeter) = bfs(position, &mut visited, &grid, plant_type);
            acc + (area.len() * perimeter)
        }
    })
}

fn total_price_fencing_sides(input: &str) -> usize {
    let grid: Grid<char> = parse_input_grid(input);
    let mut visited: HashSet<Position> = HashSet::new();

    grid.indexed_iter().fold(0, |acc, (position, &plant_type)| {
        if visited.contains(&position) {
            acc
        } else {
            let (area, _) = bfs(position, &mut visited, &grid, plant_type);
            let sides = count_sides(&area);
            // println!(
            //     "plant: {}, area: {}, sides: {}",
            //     plant_type,
            //     area.len(),
            //     sides
            // );
            acc + (area.len() * sides)
        }
    })
}

fn count_sides(area: &HashSet<Position>) -> usize {
    [(1, 0), (0, 1), (0, -1), (-1, 0)]
        .iter()
        .map(|&(dx, dy)| {
            let sides: HashSet<Position> = area
                .iter()
                .filter_map(|&(x, y)| {
                    let neighbor = (
                        x.wrapping_add(dx as isize as usize),
                        y.wrapping_add(dy as isize as usize),
                    );
                    if !area.contains(&neighbor) {
                        Some(neighbor)
                    } else {
                        None
                    }
                })
                .collect();

            sides
                .iter()
                .fold(sides.clone(), |mut sides, &side| {
                    let mut overlap = (
                        side.0.wrapping_add(dy as isize as usize),
                        side.1.wrapping_add(dx as isize as usize),
                    );
                    while sides.contains(&overlap) {
                        sides.remove(&overlap);
                        overlap = (
                            overlap.0.wrapping_add(dy as isize as usize),
                            overlap.1.wrapping_add(dx as isize as usize),
                        );
                    }
                    sides
                })
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#
    }

    #[test]
    fn test_total_price_fencing() {
        let input = test_input();
        assert_eq!(total_price_fencing(&input), 1930);
    }

    #[test]
    fn test_total_price_fencing_sides() {
        let input = test_input();
        assert_eq!(total_price_fencing_sides(&input), 1206);
    }
}
