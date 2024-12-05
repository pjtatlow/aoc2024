advent_of_code::solution!(4);

static LETTERS: [char; 4] = ['X', 'M', 'A', 'S'];

fn add(a: usize, delta: i32) -> usize {
    (a as i32 + delta) as usize
}

fn find_xmas(map: &[Vec<char>], (x, y): (usize, usize), (delta_x, delta_y): (i32, i32)) -> u32 {
    for i in 1..4 {
        let new_y = add(y, delta_y * i);
        if let Some(row) = map.get(new_y) {
            let new_x = add(x, delta_x * i);
            if let Some(letter) = row.get(new_x) {
                if letter != &LETTERS[i as usize] {
                    return 0;
                }
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    }
    1
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'X' {
                for delta_y in (-1_i32)..=1 {
                    for delta_x in -1_i32..=1 {
                        count += find_xmas(&map, (x, y), (delta_x, delta_y));
                    }
                }
            }
        }
    }

    Some(count)
}

fn is_xmas(map: &[Vec<char>], (x, y): (usize, usize)) -> bool {
    let mut a = [map[y - 1][x - 1], map[y + 1][x + 1]];
    let mut b = [map[y + 1][x - 1], map[y - 1][x + 1]];
    a.sort();
    b.sort();
    matches!((a, b), (['M', 'S'], ['M', 'S']))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count = 0;
    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if map[y][x] == 'A' && is_xmas(&map, (x, y)) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
