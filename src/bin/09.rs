use std::collections::{BTreeMap, HashMap};

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let sum = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();
    let mut drive = vec![-1_i64; sum];
    let mut index = 0_usize;
    let mut id = 0_i64;
    let mut iter = input.trim().chars();
    while let Some(size) = iter.next() {
        let length = size.to_digit(10).unwrap() as usize;
        drive[index..index + length].fill(id);
        index += length;
        id += 1;

        if let Some(empty) = iter.next() {
            index += empty.to_digit(10).unwrap() as usize;
        }
    }

    let mut next_empty = drive.iter().position(|&x| x == -1).unwrap();
    for i in (0..drive.len()).rev() {
        if i <= next_empty {
            break;
        }
        if drive[i] != -1 {
            drive.swap(next_empty, i);
            next_empty = drive[next_empty + 1..]
                .iter()
                .position(|&x| x == -1)
                .unwrap()
                + next_empty
                + 1;
        }
    }

    let checksum =
        drive.into_iter().enumerate().fold(
            0_i64,
            |acc, (x, i)| if i > 0 { acc + (x as i64 * i) } else { acc },
        );

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sum = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();
    let mut empty_slots: Vec<Vec<usize>> = vec![vec![]; 10];
    let mut drive = vec![-1_i64; sum];
    let mut index = 0_usize;
    let mut id = 0_i64;
    let mut iter = input.trim().chars();
    while let Some(size) = iter.next() {
        let length = size.to_digit(10).unwrap() as usize;
        drive[index..index + length].fill(id);
        index += length;
        id += 1;

        if let Some(empty) = iter.next() {
            let empty = empty.to_digit(10).unwrap() as usize;
            empty_slots[empty].push(index);
            index += empty;
        }
    }

    for id in (1..id).rev() {
        let start = drive.iter().position(|&x| x == id).unwrap();
        let file_length = drive[start..]
            .iter()
            .position(|&x| x != id)
            .unwrap_or(drive.len() - start);

        let empty_space = drive.iter().enumerate().position(|(i, &x)| {
            if x == -1 && i + file_length < drive.len() {
                drive[i..i + file_length].iter().all(|&x| x == -1)
            } else {
                false
            }
        });
        if let Some(empty_index) = empty_space {
            if empty_index < start {
                for i in 0..file_length {
                    drive.swap(empty_index + i, start + i);
                }
            }
        }
    }

    let checksum =
        drive.into_iter().enumerate().fold(
            0_i64,
            |acc, (x, i)| if i > 0 { acc + (x as i64 * i) } else { acc },
        );

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
