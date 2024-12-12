use std::collections::{HashMap, HashSet};

use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(10);

fn dfs(map: &[Vec<u8>], (y, x): (usize, usize), summits: &mut HashSet<(usize, usize)>) {
    let cur_value = map[y][x];
    for (delta_y, delta_x) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_y = y as i64 + delta_y;
        if new_y < 0 || new_y >= map.len() as i64 {
            continue;
        }
        let new_y = new_y as usize;
        let new_x = x as i64 + delta_x;
        if new_x < 0 || new_x >= map[new_y].len() as i64 {
            continue;
        }
        let new_x = new_x as usize;

        if cur_value + 1 == map[new_y][new_x] {
            if map[new_y][new_x] == 9 {
                summits.insert((new_y, new_x));
            } else {
                dfs(map, (new_y, new_x), summits);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let mut num_trailheads = 0_u32;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let mut summits = HashSet::new();
                dfs(&map, (y, x), &mut summits);
                num_trailheads += summits.len() as u32;
            }
        }
    }
    Some(num_trailheads)
}

fn dfs2(
    map: &[Vec<u8>],
    (y, x): (usize, usize),
    path: Vec<(usize, usize)>,
    paths: &mut HashSet<Vec<(usize, usize)>>,
) {
    let mut path = path.clone();
    path.push((y, x));
    let cur_value = map[y][x];
    for (delta_y, delta_x) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_y = y as i64 + delta_y;
        if new_y < 0 || new_y >= map.len() as i64 {
            continue;
        }
        let new_y = new_y as usize;
        let new_x = x as i64 + delta_x;
        if new_x < 0 || new_x >= map[new_y].len() as i64 {
            continue;
        }
        let new_x = new_x as usize;

        if cur_value + 1 == map[new_y][new_x] {
            if map[new_y][new_x] == 9 {
                let mut complete_path = path.clone();
                complete_path.push((new_y, new_x));

                paths.insert(complete_path);
            } else {
                dfs2(map, (new_y, new_x), path.clone(), paths);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        u8::MAX
                    } else {
                        c.to_digit(10).unwrap() as u8
                    }
                })
                .collect()
        })
        .collect();
    let mut num_paths = 0_u32;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let mut paths = HashSet::new();
                dfs2(&map, (y, x), Vec::new(), &mut paths);
                println!("Found {} paths at ({}, {})", paths.len(), y, x);
                num_paths += paths.len() as u32;
            }
        }
    }
    Some(num_paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
