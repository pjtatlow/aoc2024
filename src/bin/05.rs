advent_of_code::solution!(5);

fn rule_check(pages: &[&str], rules: &[(&str, &str)]) -> bool {
    for (a, b) in rules {
        let a_index = pages.iter().position(|&s| s == *a);
        let b_index = pages.iter().position(|&s| s == *b);
        if let (Some(i), Some(j)) = (a_index, b_index) {
            if i > j {
                return false;
            }
        }
    }
    true
}

fn fix_pages(pages: &mut [&str], rules: &[(&str, &str)]) -> bool {
    for (a, b) in rules {
        let a_index = pages.iter().position(|&s| s == *a);
        let b_index = pages.iter().position(|&s| s == *b);
        if let (Some(i), Some(j)) = (a_index, b_index) {
            if i > j {
                pages.swap(i, j);
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let rules: Vec<(&str, &str)> = top
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .collect();

    let mut total = 0_u32;

    for line in bottom.lines() {
        let pages: Vec<&str> = line.split(',').collect();
        if rule_check(&pages, &rules) {
            total += pages[pages.len() / 2].parse::<u32>().unwrap();
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let rules: Vec<(&str, &str)> = top
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .collect();

    let mut total = 0_u32;

    for line in bottom.lines() {
        let mut pages: Vec<&str> = line.split(',').collect();
        if !rule_check(&pages, &rules) {
            while !rule_check(&pages, &rules) {
                fix_pages(&mut pages, &rules);
            }
            total += pages[pages.len() / 2].parse::<u32>().unwrap();
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
