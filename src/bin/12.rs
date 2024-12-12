use std::collections::{BTreeMap, BTreeSet, HashMap};

advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Default, Clone)]
struct Cost {
    area: usize,
    perimeter: usize,
    sides: BTreeSet<(Side, (usize, usize))>,
}

impl Cost {
    fn reduce(mut self) -> Self {
        let sides = self.sides.clone();
        for (side, (y, x)) in sides {
            match side {
                Side::Left | Side::Right => {
                    if let Some(new_y) = y.checked_sub(1) {
                        if self.sides.contains(&(side, (new_y, x))) {
                            self.sides.remove(&(side, (y, x)));
                        }
                    }
                    if self.sides.contains(&(side, (y + 1, x))) {
                        self.sides.remove(&(side, (y, x)));
                    }
                }
                Side::Top | Side::Bottom => {
                    if let Some(new_x) = x.checked_sub(1) {
                        if self.sides.contains(&(side, (y, new_x))) {
                            self.sides.remove(&(side, (y, x)));
                        }
                    }
                    if self.sides.contains(&(side, (y, x + 1))) {
                        self.sides.remove(&(side, (y, x)));
                    }
                }
            }
        }
        self
    }
}

enum Increment {
    Pos(usize),
    Neg(usize),
}

impl Increment {
    fn apply(&self, x: usize) -> Option<usize> {
        match self {
            Self::Pos(y) => Some(x + *y),
            Self::Neg(y) => x.checked_sub(*y),
        }
    }
}

const NEIGHBOR_DIRECTIONS: [(Increment, Increment, Side); 4] = [
    (Increment::Pos(1), Increment::Pos(0), Side::Top),
    (Increment::Pos(0), Increment::Pos(1), Side::Right),
    (Increment::Neg(1), Increment::Pos(0), Side::Bottom),
    (Increment::Pos(0), Increment::Neg(1), Side::Left),
];

#[derive(Debug, Default)]
struct Data {
    id_letters: HashMap<usize, char>,
    ids: HashMap<(usize, usize), (usize, char)>,
    costs: BTreeMap<usize, Cost>,
    id: usize,
    map: Vec<Vec<char>>,
}

fn handle_pos(data: &mut Data, y: usize, x: usize) {
    let letter = data.map[y][x];
    let neighbors = NEIGHBOR_DIRECTIONS
        .iter()
        .filter_map(
            |(delta_y, delta_x, side)| match (delta_y.apply(y), delta_x.apply(x)) {
                (Some(y), Some(x)) => {
                    if let Some(&c) = data.map.get(y).and_then(|row| row.get(x)) {
                        Some(((y, x), c, side))
                    } else {
                        None
                    }
                }
                _ => None,
            },
        )
        .collect::<Vec<_>>();
    let id = match neighbors.iter().find_map(|(pos, c, _)| {
        if *c != letter {
            return None;
        }
        if let Some((id, _)) = data.ids.get(pos) {
            return Some(*id);
        }
        None
    }) {
        Some(id) => id,
        None => {
            data.id += 1;
            data.id_letters.insert(data.id, letter);
            data.id
        }
    };
    data.ids.insert((y, x), (id, letter));
    let cost = data.costs.entry(id).or_default();
    cost.area += 1;
    cost.perimeter += 4 - neighbors.iter().filter(|(_, c, _)| *c == letter).count();
    for side in [Side::Top, Side::Right, Side::Bottom, Side::Left] {
        if let Some((_, c, _)) = neighbors.iter().find(|(_, _, s)| **s == side) {
            if *c != letter {
                cost.sides.insert((side, (y, x)));
            }
        } else {
            cost.sides.insert((side, (y, x)));
        }
        // if !neighbors
        //     .iter()
        //     .any(|(_, c, s)| *c == letter && **s == side)
        // {
        //     cost.sides.insert((side, (y, x)));
        // }
    }

    let relatives = neighbors
        .iter()
        .filter(|(_, c, _)| *c == letter)
        .collect::<Vec<_>>();

    for (n, _, _) in relatives {
        if !data.ids.contains_key(n) {
            handle_pos(data, n.0, n.1);
            // q.push(*n);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut data = Data {
        map,
        ..Default::default()
    };
    for y in 0..data.map.len() {
        for x in 0..data.map[y].len() {
            if !data.ids.contains_key(&(y, x)) {
                handle_pos(&mut data, y, x);
            }
        }
    }

    Some(
        data.costs
            .values()
            .fold(0, |count, cost| count + cost.area * cost.perimeter),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut data = Data {
        map,
        ..Default::default()
    };
    for y in 0..data.map.len() {
        for x in 0..data.map[y].len() {
            if !data.ids.contains_key(&(y, x)) {
                handle_pos(&mut data, y, x);
            }
        }
    }

    Some(
        data.costs
            .into_values()
            .map(|c| c.reduce())
            .fold(0, |count, cost| count + cost.area * cost.sides.len()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
