use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }
    left.sort();
    right.sort();
    let mut sum = 0_u32;
    for (left, right) in left.iter().zip(right.iter()) {
        sum += (left - right).unsigned_abs();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = vec![];
    let mut right: HashMap<i32, u32> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        left.push(parts[0].parse().unwrap());
        *right.entry(parts[1].parse().unwrap()).or_insert(0) += 1;
    }
    let mut sum = 0_u32;
    for left in left {
        let right_count = if let Some(right) = right.get(&left) {
            *right
        } else {
            0
        };
        sum += left as u32 * right_count;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
