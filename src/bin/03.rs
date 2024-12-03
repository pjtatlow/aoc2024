use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0_u32;
    for m in re.captures_iter(input) {
        let (_, [a, b]) = m.extract();
        sum += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|((d)(o)\(\))|((do)(n't)\(\))").unwrap();
    let mut sum = 0_u32;
    let mut enabled = true;
    for m in re.captures_iter(input) {
        match m.get(0).unwrap().as_str() {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if enabled {
                    let (_, [_, a, b]) = m.extract();
                    sum += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
                }
            }
        }
        // m.
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
