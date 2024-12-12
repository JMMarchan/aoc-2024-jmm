use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day09.txt").expect("Day 9 input file should be present");
    let sol1 = get_checksum(&input);
    let sol2 = get_checksum_whole_file_defrag(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_checksum(input: &str) -> u64 {
    let dense_disk: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let total_size: usize = dense_disk.iter().sum::<u32>() as usize;
    let mut disk: Vec<Option<u32>> = vec![None; total_size];

    let mut id = 0;
    let mut index = 0;
    let mut free = false;

    for length in dense_disk {
        if !free {
            for i in index..index + length as usize {
                disk[i] = Some(id);
            }
            id += 1;
        }
        index += length as usize;
        free = !free;
    }

    let mut left_index = disk.iter().position(|&x| x.is_none()).unwrap_or(0);
    let mut right_index = disk.len() - 1;

    while right_index > left_index {
        if let Some(file_id) = disk[right_index] {
            disk[left_index] = Some(file_id);
            disk[right_index] = None;
            left_index = disk
                .iter()
                .position(|&x| x.is_none())
                .unwrap_or(left_index + 1);
        }
        right_index -= 1;
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, &block)| block.map(|file_id| (i as u64) * file_id as u64))
        .sum()
}

fn get_checksum_whole_file_defrag(input: &str) -> u64 {
    let dense_disk: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let total_size: usize = dense_disk.iter().sum::<u32>() as usize;

    let mut disk: Vec<Option<u32>> = vec![None; total_size];

    let mut id = 0;
    let mut index = 0;
    let mut free = false;

    for length in dense_disk {
        if !free {
            for i in index..index + length as usize {
                disk[i] = Some(id);
            }
            id += 1;
        }
        index += length as usize;
        free = !free;
    }

    let mut free_spaces: Vec<(usize, usize)> = Vec::new(); // Vec<(pos, size)>
    let mut files: Vec<(usize, usize, u32)> = Vec::new(); // Vec<(pos, size, file_id)>
    let mut pos = 0;

    while pos < disk.len() {
        let start = pos;
        let value = disk[start];
        while pos < disk.len() && disk[pos] == value {
            pos += 1;
        }

        let size = pos - start;
        if value.is_none() {
            free_spaces.push((start, size));
        } else if let Some(file_id) = value {
            files.push((start, size, file_id));
        }
    }

    files.sort_by_key(|&(_, _, file_id)| file_id);
    for &(f_pos, f_size, file_id) in files.iter().rev() {
        for free_space in free_spaces.iter_mut() {
            let (free_pos, free_size) = *free_space;
            if free_pos >= f_pos {
                break;
            }
            if free_size >= f_size {
                for i in 0..f_size {
                    disk[free_pos + i] = Some(file_id);
                }
                for i in 0..f_size {
                    disk[f_pos + i] = None;
                }
                *free_space = (free_pos + f_size, free_size - f_size);
                break;
            }
        }
    }

    // println!("disk: {:?}", disk);

    disk.iter()
        .enumerate()
        .filter_map(|(i, &block)| block.map(|file_id| (i as u64) * file_id as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"2333133121414131402"#
    }

    // input represents 00...111...2...333.44.5555.6666.777.888899
    // once files are moved
    // input becomes 0099811188827773336446555566..............
    #[test]
    fn test_get_checksum() {
        let input = test_input();
        assert_eq!(get_checksum(&input), 1928);
    }

    // 00992111777.44.333....5555.6666.....8888..
    #[test]
    fn test_get_checksum_whole_file_defrag() {
        let input = test_input();
        assert_eq!(get_checksum_whole_file_defrag(&input), 2858);
    }
}
