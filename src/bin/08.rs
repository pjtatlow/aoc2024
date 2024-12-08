use std::collections::{HashMap, HashSet};

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = map.len();
    let width = map[0].len();
    let nodes: HashMap<char, Vec<(usize, usize)>> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut map, (y, line)| {
                for (x, c) in line.char_indices() {
                    if c != '.' {
                        map.entry(c).or_default().push((y, x));
                    }
                }
                map
            });
    let mut anti_nodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, nodes) in nodes {
        for &start in nodes.iter() {
            for &end in nodes.iter() {
                if start == end {
                    continue;
                }
                let delta_y = start.0 as i32 - end.0 as i32;
                let new_y = start.0 as i32 + delta_y;
                if new_y < 0 || new_y >= height as i32 {
                    continue;
                }
                let delta_x = start.1 as i32 - end.1 as i32;
                let new_x = start.1 as i32 + delta_x;
                if new_x < 0 || new_x >= width as i32 {
                    continue;
                }

                anti_nodes.insert((new_y as usize, new_x as usize));
            }
        }
    }

    Some(anti_nodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = map.len();
    let width = map[0].len();
    let nodes: HashMap<char, Vec<(usize, usize)>> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut map, (y, line)| {
                for (x, c) in line.char_indices() {
                    if c != '.' {
                        map.entry(c).or_default().push((y, x));
                    }
                }
                map
            });
    let mut anti_nodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, nodes) in nodes {
        for &start in nodes.iter() {
            anti_nodes.insert(start);
            for &end in nodes.iter() {
                if start == end {
                    continue;
                }
                let mut start = start;
                let mut end = end;
                loop {
                    let delta_y = start.0 as i32 - end.0 as i32;
                    let new_y = start.0 as i32 + delta_y;
                    if new_y < 0 || new_y >= height as i32 {
                        break;
                    }
                    let delta_x = start.1 as i32 - end.1 as i32;
                    let new_x = start.1 as i32 + delta_x;
                    if new_x < 0 || new_x >= width as i32 {
                        break;
                    }
                    end = start;
                    start = (new_y as usize, new_x as usize);

                    anti_nodes.insert(start);
                }
            }
        }
    }

    Some(anti_nodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
