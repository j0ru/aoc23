use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum GridItem {
    Number {
        value: u32,
        len: u32,
        id: u32,
        is_first: bool,
    },
    Symbol {
        value: char,
    },
}

#[derive(Debug)]
pub struct Schematic {
    items: HashMap<(u32, u32), GridItem>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Schematic {
    let mut res = HashMap::new();
    let re = Regex::new(r"\d+").unwrap();
    let mut id = 0;
    for (y, line) in input.lines().enumerate() {
        let line = line.trim();
        for item in re.find_iter(line) {
            for i in 0..item.as_str().len() {
                res.insert(
                    (item.start() as u32 + i as u32, y as u32),
                    GridItem::Number {
                        value: item.as_str().parse().unwrap(),
                        len: item.as_str().len() as u32,
                        id,
                        is_first: i == 0,
                    },
                );
            }
            id += 1;
        }
    }
    for (index_y, line) in input.lines().enumerate() {
        let line = line.trim();
        for (index_x, c) in line.chars().enumerate() {
            if c != '.' && !c.is_ascii_digit() {
                res.insert(
                    (index_x as u32, index_y as u32),
                    GridItem::Symbol { value: c },
                );
            }
        }
    }

    Schematic { items: res }
}

#[aoc(day3, part1)]
fn solve_part1(input: &Schematic) -> u32 {
    let mut res = 0;
    let mut found_ids = Vec::new();
    for (loc, item) in input.items.iter() {
        if let GridItem::Number {
            value,
            len,
            id,
            is_first,
        } = item
        {
            if !found_ids.contains(&id) && *is_first {
                found_ids.push(id);
                'outer: for x in loc.0.saturating_sub(1)..=loc.0 + len {
                    for y in loc.1.saturating_sub(1)..=loc.1 + 1 {
                        if matches!(input.items.get(&(x, y)), Some(GridItem::Symbol { .. })) {
                            res += value;
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    res
}

#[aoc(day3, part2)]
fn solve_part2(input: &Schematic) -> u32 {
    let mut res = 0;

    for (loc, item) in input.items.iter() {
        if let GridItem::Symbol { value: '*' } = item {
            let mut found_ids = HashMap::new();
            for x in loc.0.saturating_sub(1)..=loc.0 + 1 {
                for y in loc.1.saturating_sub(1)..=loc.1 + 1 {
                    if let Some(GridItem::Number { value, id, .. }) = input.items.get(&(x, y)) {
                        found_ids.insert(id, *value);
                    }
                }
            }
            if found_ids.len() == 2 {
                res += found_ids.into_values().reduce(|acc, x| x * acc).unwrap()
            }
        }
    }

    res
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        const EXAMPLE: &'static str = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 4361);
    }

    #[test]
    fn example_part2() {
        const EXAMPLE: &'static str = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 467835);
    }
}
