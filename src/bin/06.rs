use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(6);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let y = map.iter().position(|row| row.contains(&'^')).unwrap();
    let x = map[y].iter().position(|c| c == &'^').unwrap();

    let mut pos = Some((y, x));
    let mut dir = Direction::Up;
    while let Some((y, x)) = pos {
        let next_pos = match dir {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        };
        if let Some(next_row) = map.get_mut(next_pos.0) {
            if let Some(&c) = next_row.get(next_pos.1) {
                match c {
                    '#' => {
                        // TURN RIGHT
                        dir = match dir {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        }
                    }
                    _ => {
                        // CHANGE TILE
                        next_row[next_pos.1] = '^';
                        pos = Some(next_pos);
                    }
                }
            } else {
                pos = None;
            }
        } else {
            pos = None;
        }
    }

    Some(map.iter().fold(0_u32, |acc, row| {
        acc + row.iter().filter(|&&c| c == '^').count() as u32
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let original_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let y = original_map
        .iter()
        .position(|row| row.contains(&'^'))
        .unwrap();
    let x = original_map[y].iter().position(|c| c == &'^').unwrap();

    let mut total = 0;
    for i in 0..original_map.len() {
        for j in 0..original_map[i].len() {
            if original_map[i][j] != '.' {
                continue;
            }

            let mut map: Vec<Vec<char>> = original_map.clone();
            map[i][j] = '#';

            let mut cycle = false;
            let mut dir = Direction::Up;
            let mut pos = Some((y, x));
            while let Some((y, x)) = pos {
                let next_pos = match dir {
                    Direction::Up => {
                        if y > 0 {
                            (y - 1, x)
                        } else {
                            break;
                        }
                    }
                    Direction::Down => (y + 1, x),
                    Direction::Left => {
                        if x > 0 {
                            (y, x - 1)
                        } else {
                            break;
                        }
                    }
                    Direction::Right => (y, x + 1),
                };
                if let Some(next_row) = map.get_mut(next_pos.0) {
                    if let Some(&c) = next_row.get(next_pos.1) {
                        match c {
                            '#' => {
                                // TURN RIGHT
                                dir = match dir {
                                    Direction::Up => Direction::Right,
                                    Direction::Down => Direction::Left,
                                    Direction::Left => Direction::Up,
                                    Direction::Right => Direction::Down,
                                }
                            }
                            _ => {
                                // CHANGE TILE
                                let next_tile = match dir {
                                    Direction::Up => '^',
                                    Direction::Down => 'v',
                                    Direction::Left => '<',
                                    Direction::Right => '>',
                                };

                                if c == next_tile {
                                    cycle = true;
                                    break;
                                }

                                next_row[next_pos.1] = next_tile;
                                pos = Some(next_pos);
                            }
                        }
                    } else {
                        pos = None;
                    }
                } else {
                    pos = None;
                }
            }

            if cycle {
                total += 1;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
