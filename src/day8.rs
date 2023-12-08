use nom::{
    bytes::complete::{is_a, tag},
    character::complete::alphanumeric1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use std::time::Instant;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

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
    name: &'a str,
    left: Weak<RefCell<Self>>,
    right: Weak<RefCell<Self>>,
}

impl<'a> Node<'a> {
    pub fn get(&self, direction: Direction) -> Weak<RefCell<Self>> {
        match direction {
            Direction::Left => self.left.clone(),
            Direction::Right => self.right.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map<'a> {
    nodes: Vec<Rc<RefCell<Node<'a>>>>,
    current_node: Weak<RefCell<Node<'a>>>,
    instructions: Instructions,
}

impl<'a> Iterator for Map<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.current_node.upgrade().unwrap();
        let next_node = current_node.borrow().get(self.instructions.next().unwrap());

        self.current_node = next_node;
        Some(self.current_node.upgrade().unwrap().borrow().name)
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
            nodes: Vec::new(),
            current_node: Weak::new(),
            instructions: Instructions {
                directions,
                current: 0,
            },
        };

        let mut node_map = HashMap::new();

        for line in line_iter.skip(1) {
            let (name, rest) = line.split_once(" = ").unwrap();
            res.nodes.push(Rc::new(RefCell::new(Node {
                name,
                left: Weak::new(),
                right: Weak::new(),
            })));
            if let Ok((_, (left, right))) = parse_moves(rest) {
                node_map.insert(name, (res.nodes.len() - 1, (left, right)));
            }
        }

        for (node_idx, (left, right)) in node_map.values() {
            let left = Rc::downgrade(res.nodes.get(node_map.get(left).unwrap().0).unwrap());
            let right = Rc::downgrade(res.nodes.get(node_map.get(right).unwrap().0).unwrap());

            let current = res.nodes.get_mut(*node_idx).unwrap();
            current.borrow_mut().left = left;
            current.borrow_mut().right = right;
        }

        res.current_node = Rc::downgrade(res.nodes.get(node_map.get("AAA").unwrap().0).unwrap());

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

const SOLUTION_PART2: f64 = 15746133679061.0;

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> Number {
    let map = Map::from(input);
    let mut routes: Vec<Map> = map
        .nodes
        .iter()
        .filter_map(|node| {
            if node.borrow().name.ends_with('A') {
                let mut res = map.clone();
                res.current_node = Rc::downgrade(node);
                Some(res)
            } else {
                None
            }
        })
        .collect();

    let start = Instant::now();
    let mut res = 0;
    loop {
        res += 1;
        if res % 10_000_000 == 0 && start.elapsed().as_secs() != 0 {
            let progress = res as f64 / SOLUTION_PART2;
            println!(
                "Iteration\t{res}\t\t{:.5}%\t\t{}it/s\t\t{:.0}h left",
                progress * 100.,
                crate::common::human_readable_numbers(res / start.elapsed().as_secs()),
                start.elapsed().as_secs() as f64 / 3600. / progress
            );
        }

        let mut at_end = 0;
        for route in routes.iter_mut() {
            let current = route.next().unwrap();
            if current.ends_with('Z') {
                at_end += 1;
            }
        }
        if at_end == routes.len() {
            break;
        }
    }

    res
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
