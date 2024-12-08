use aoc_parse::{parser, prelude::*};

advent_of_code::solution!(7);

enum Operator {
    Add,
    Mul,
    Cat,
}

fn is_possible(operands: &[u64], target: u64, operators: &[Operator]) -> bool {
    if operands.len() == 1 {
        return operands[0] == target;
    }
    if operands[0] > target {
        return false;
    }

    for operator in operators {
        let left = operands[0];
        let right = operands[1];
        let result = match operator {
            Operator::Add => left + right,
            Operator::Mul => {
                if let Some(v) = left.checked_mul(right) {
                    v
                } else {
                    continue;
                }
            }
            Operator::Cat => format!("{}{}", left, right).parse().unwrap(),
        };
        let new = if operands.len() > 2 {
            let mut new_operands = operands[2..].to_vec();

            new_operands.insert(0, result);
            new_operands
        } else {
            vec![result]
        };

        if is_possible(&new, target, operators) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (total, parts) = line.split_once(':').unwrap();
                let total: u64 = total.parse().unwrap();
                let operands: Vec<u64> = parts
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                (total, operands)
            })
            .filter(|(total, operands)| {
                is_possible(operands, *total, &[Operator::Add, Operator::Mul])
            })
            .fold(0_u64, |acc, (total, _)| acc + total),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (total, parts) = line.split_once(':').unwrap();
                let total: u64 = total.parse().unwrap();
                let operands: Vec<u64> = parts
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                (total, operands)
            })
            .filter(|(total, operands)| {
                is_possible(
                    operands,
                    *total,
                    &[Operator::Add, Operator::Mul, Operator::Cat],
                )
            })
            .fold(0_u64, |acc, (total, _)| acc + total),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
