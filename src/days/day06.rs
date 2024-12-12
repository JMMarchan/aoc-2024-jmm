use crate::{Solution, SolutionPair};
use grid::*;
use hashbrown::{HashMap, HashSet};
use rayon::prelude::*;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day06.txt").expect("Day 6 input file should be present");
    let visited_spaces: HashSet<Position> = get_distinct_visited(&input);
    let sol1: u32 = visited_spaces.len() as u32;
    let sol2: u32 = count_looping_obstructions(&input, visited_spaces);

    (Solution::from(sol1), Solution::from(sol2))
}

type Position = (usize, usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_step(&self, (i, j): Position) -> Position {
        match self {
            Direction::North => (i.checked_sub(1).unwrap_or(usize::MAX), j),
            Direction::East => (i, j + 1),
            Direction::South => (i + 1, j),
            Direction::West => (i, j.checked_sub(1).unwrap_or(usize::MAX)),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn parse_input_grid(input: &str) -> Grid<char> {
    let grid_vec: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    let grid_cols = input.find('\n').unwrap_or(input.len());
    Grid::from_vec(grid_vec.clone(), grid_cols)
}

fn get_distinct_visited(input: &str) -> HashSet<Position> {
    let grid = parse_input_grid(input);

    let mut visited_spaces: HashSet<Position> = HashSet::new();

    // Find the starting position of the guard
    let guard_start = grid
        .indexed_iter()
        .find_map(|((i, j), &char)| if char == '^' { Some((i, j)) } else { None })
        .unwrap();

    visited_spaces.insert(guard_start);

    let mut guard: (Position, Direction) = (guard_start, Direction::North);

    loop {
        let (current_pos, current_dir) = guard;

        let next_pos = current_dir.next_step(current_pos);

        match grid.get(next_pos.0, next_pos.1) {
            Some(&'#') => {
                // turn right
                guard = (current_pos, current_dir.turn_right());
            }
            Some(_) => {
                // move forward
                guard = (next_pos, current_dir);
                visited_spaces.insert(next_pos);
            }
            None => {
                // out of bounds
                break;
            }
        }
    }

    visited_spaces
}

// count the number of looping paths if we place one obstruction
// TODO: SLOW SLOW SLOW, 300ms EWWW
fn count_looping_obstructions(input: &str, mut visited_spaces: HashSet<Position>) -> u32 {
    let grid = parse_input_grid(input);

    let guard_start = grid
        .indexed_iter()
        .find_map(|((i, j), &char)| if char == '^' { Some((i, j)) } else { None })
        .unwrap();

    // We use visited_spaces from part 1 to optimize, only try placing obstructions from visited_spaces

    let guard: (Position, Direction) = (guard_start, Direction::North);

    visited_spaces.remove(&guard.0);

    visited_spaces
        .par_iter()
        .filter(|&obstruction| check_looping_path(*obstruction, guard, &grid))
        .count() as u32
}

fn check_looping_path(
    obstruction: Position,
    guard: (Position, Direction),
    grid: &Grid<char>,
) -> bool {
    let mut grid = grid.clone(); // Clone the grid to avoid modifying the original
                                 // Place the obstruction in the grid
    if let Some(cell) = grid.get_mut(obstruction.0, obstruction.1) {
        *cell = '#';
    }

    let mut visited_states: HashSet<(Position, Direction)> = HashSet::new();
    let mut current_guard = guard;

    loop {
        let (current_pos, current_dir) = current_guard;

        // If this state was visited before, we have a loop
        if !visited_states.insert((current_pos, current_dir)) {
            return true;
        }

        // Calculate the next position
        let next_pos = current_dir.next_step(current_pos);

        match grid.get(next_pos.0, next_pos.1) {
            Some(&'#') => {
                // Turn right on obstruction
                current_guard = (current_pos, current_dir.turn_right());
            }
            Some(_) => {
                // Move forward if the path is clear
                current_guard = (next_pos, current_dir);
            }
            None => {
                // Out of bounds, terminate
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#
    }

    #[test]
    fn test_count_distinct_visited() {
        let input = test_input();
        assert_eq!(get_distinct_visited(&input).len(), 41);
    }

    #[test]
    fn test_count_looping_obstructions() {
        let input = test_input();
        assert_eq!(
            count_looping_obstructions(&input, get_distinct_visited(&input)),
            6
        );
    }
}
