use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut stones: Vec<u128> = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<u128>().unwrap())
        .collect();
    for _ in 0..25 {
        stones = stones.into_iter().fold(Vec::new(), |mut acc, stone| {
            if stone == 0 {
                acc.push(1);
                return acc;
            }
            let s = stone.to_string();
            if s.len() % 2 == 0 {
                let (a, b) = s.split_at(s.len() / 2);
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                acc.push(a);
                acc.push(b);
                return acc;
            }

            acc.push(stone * 2024);

            acc
        })
    }
    Some(stones.len() as u64)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut memos: HashMap<u128, Vec<(u128, usize)>> = HashMap::new();
    let mut stones: Vec<(u128, usize)> = input
        .split_ascii_whitespace()
        .map(|x| (x.parse::<u128>().unwrap(), 0))
        .collect();
    println!("STONES: {:?}", stones);

    let mut num_stones = 0;
    let mut i = 0;
    while i < stones.len() {
        let (n, end) = stones[i];
        let memo = memos.entry(n).or_default();
        let ((mut stone, mut count), start) = match memo.last() {
            Some(&x) => (x, memo.len()),
            None => ((n, 1), end),
        };

        for x in start..5 {
            if end == 0 && x == memo.len() {
                memo.push((stone, count));
            }
            count += 1;

            if stone == 0 {
                stone = 1;
                continue;
            }

            let s = stone.to_string();
            if s.len() % 2 == 0 {
                let (a, b) = s.split_at(s.len() / 2);
                let a = a.parse().unwrap();
                stone = a;
                let b = b.parse().unwrap();
                stones.push((b, count));
                continue;
            }
            stone *= 2024;
        }
        num_stones += count;
        i += 1;
    }
    println!("{:?}", memos.get(&0));

    Some(num_stones as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
