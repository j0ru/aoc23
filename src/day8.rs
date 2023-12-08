use nom::{
    bytes::complete::{is_a, tag},
    character::complete::alphanumeric1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use std::collections::HashMap;

use aoc_runner_derive::aoc;

type Number = u64;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Instructions {
    directions: Vec<Direction>,
    current: usize,
}

impl Iterator for Instructions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.directions.get(self.current);
        self.current = (self.current + 1) % self.directions.len();

        res.cloned()
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    pub fn get(&self, direction: Direction) -> &'a str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
    current_node: &'a str,
    instructions: Instructions,
}

impl<'a> Iterator for Map<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.nodes.get(self.current_node)?;
        let next_node = current_node.get(self.instructions.next().unwrap());

        self.current_node = next_node;
        Some(self.current_node)
    }
}

impl<'a> Map<'a> {
    fn from(value: &'a str) -> Self {
        let mut line_iter = value.lines();
        let directions: Vec<Direction> = line_iter
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!("unexpected character in direction input"),
            })
            .collect();

        let mut res = Map {
            nodes: HashMap::new(),
            current_node: "AAA",
            instructions: Instructions {
                directions,
                current: 0,
            },
        };

        for line in line_iter.skip(1) {
            let (name, rest) = line.split_once(" = ").unwrap();
            if let Ok((_, (left, right))) = parse_moves(rest) {
                res.nodes.insert(name, Node { left, right });
            }
        }

        res
    }
}

fn parse_moves(line: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(left_str, tag(", "), right_str).parse(line)
}

fn left_str(line: &str) -> IResult<&str, &str> {
    is_a("(").precedes(alphanumeric1).parse(line)
}

fn right_str(line: &str) -> IResult<&str, &str> {
    terminated(alphanumeric1, tag(")")).parse(line)
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> Number {
    let map = Map::from(input);
    let mut res = 0;
    for node in map {
        res += 1;
        if node == "ZZZ" {
            break;
        }
    }

    res
}

fn vec_lcm(mut input: Vec<Number>) -> Number {
    if input.len() == 2 {
        num::integer::lcm(input[0], input[1])
    } else {
        num::integer::lcm(input.pop().unwrap(), vec_lcm(input))
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> Number {
    let map = Map::from(input);
    let routes = map
        .nodes
        .keys()
        .filter_map(|node| {
            if node.ends_with('A') {
                let mut res = map.clone();
                res.current_node = node;
                Some(res)
            } else {
                None
            }
        })
        .map(|map| {
            let mut res = 0;
            for node in map {
                res += 1;
                if node.ends_with('Z') {
                    break;
                }
            }
            res
        })
        .collect();

    vec_lcm(routes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(solve_part1(input), 2);

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(solve_part1(input), 6);
    }

    #[test]
    pub fn example_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(solve_part2(input), 6);
    }
}
