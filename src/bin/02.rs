use std::ops::Sub;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let numbers: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0_u32;
    for nums in numbers {
        if test_report(&nums) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0_u32;
    for nums in numbers {
        let valid = test_report(&nums);
        if valid {
            count += 1;
        } else {
            for i in 0..nums.len() {
                let copy = remove_index(&nums, i);
                if test_report(&copy) {
                    count += 1;
                    break;
                }
            }
        }
    }

    Some(count)
}

fn test_report(nums: &[u32]) -> bool {
    let cmp = nums[0].cmp(&nums[1]);
    let mut valid = true;
    for (i, n) in nums.iter().enumerate().skip(1) {
        let prev = nums[i - 1];
        let diff = (*n as i32).sub(prev as i32).abs();
        if prev.cmp(n) != cmp || !(1..=3).contains(&diff) {
            valid = false;
            break;
        }
    }
    valid
}

fn remove_index(nums: &[u32], index: usize) -> Vec<u32> {
    let mut new = nums.to_vec();
    new.remove(index);
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
