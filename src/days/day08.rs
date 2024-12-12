use crate::{Solution, SolutionPair};
use grid::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::*;
use std::fmt;
use std::fs::read_to_string;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day08.txt").expect("Day 8 input file should be present");
    let sol1: u32 = count_antinodes(&input);
    let sol2: u32 = count_resonant_antinodes(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

type Position = (usize, usize);

fn parse_input_grid(input: &str) -> Grid<char> {
    let grid_vec: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    let grid_cols = input.find('\n').unwrap_or(input.len());
    Grid::from_vec(grid_vec.clone(), grid_cols)
}

// an antinode is a unique position within the grid bounds
// relative to two frequencies such that the distance to one of the frequencies
// is twice the distance between the two frequences
// ie for f=(x, y), f'=(x', y'), a=(x+2dx, y+2dy) where (dx, dy)=(x-x', y-y')
// for each pair of frequencies there are two possible antinodes
fn count_antinodes(input: &str) -> u32 {
    let grid = parse_input_grid(input);

    let mut antennas: HashMap<char, HashSet<Position>> = HashMap::new();

    grid.indexed_iter()
        .filter(|(_, char)| char.is_alphanumeric())
        .for_each(|(position, &char)| {
            antennas.entry(char).or_default().insert(position);
        });

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_frequency, positions) in &antennas {
        for &pos1 in positions {
            for &pos2 in positions {
                if pos1 == pos2 {
                    continue;
                }

                let (dx, dy) = (
                    pos2.0 as isize - pos1.0 as isize,
                    pos2.1 as isize - pos1.1 as isize,
                );

                let antinode = (
                    (pos1.0 as isize + 2 * dx) as usize,
                    (pos1.1 as isize + 2 * dy) as usize,
                );

                if grid.get(antinode.0, antinode.1).is_some() {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes.len() as u32
}

fn count_resonant_antinodes(input: &str) -> u32 {
    let grid = parse_input_grid(input);

    let mut antennas: HashMap<char, HashSet<Position>> = HashMap::new();

    grid.indexed_iter()
        .filter(|(_, char)| char.is_alphanumeric())
        .for_each(|(position, &char)| {
            antennas.entry(char).or_default().insert(position);
        });

    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_frequency, positions) in &antennas {
        for &pos1 in positions {
            for &pos2 in positions {
                if pos1 == pos2 {
                    continue; // Skip self-pairs
                }

                let (dx, dy) = (
                    pos2.0 as isize - pos1.0 as isize,
                    pos2.1 as isize - pos1.1 as isize,
                );

                let mut multiplier = 1;
                loop {
                    let antinode = (
                        (pos1.0 as isize + multiplier * dx) as usize,
                        (pos1.1 as isize + multiplier * dy) as usize,
                    );

                    if grid.get(antinode.0, antinode.1).is_none() {
                        break;
                    }

                    // println!(
                    //     "pos1: {:?}, pos2: {:?}, antinode: {:?}, mult: {:?}",
                    //     pos1, pos2, antinode, multiplier
                    // );

                    antinodes.insert(antinode);
                    multiplier += 1;
                }
            }
        }
    }

    let mut new_grid = grid.clone();

    for (row, col) in &antinodes {
        if let Some(cell) = new_grid.get_mut(*row, *col) {
            *cell = '#';
        }
    }

    // println!("antinodes: {:?}", antinodes);
    // print_grid(&new_grid);

    antinodes.len() as u32
}

// fn print_grid(grid: &Grid<char>) {
//     for row in 0..grid.rows() {
//         for col in 0..grid.cols() {
//             print!("{}", grid.get(row, col).unwrap_or(&'.')); // Default to '.' if out of bounds
//         }
//         println!();
//     }
//     println!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#
    }

    #[test]
    fn test_count_antinodes() {
        let input = test_input();
        assert_eq!(count_antinodes(&input), 14);
    }

    #[test]
    fn test_count_resonant_antinodes() {
        let input = test_input();
        assert_eq!(count_resonant_antinodes(&input), 34);
    }
}
