use nom::{
    bytes::complete::{is_a, tag},
    character::complete::alphanumeric1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use nom_supreme::ParserExt;
use std::{
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
        let res = self.directions[self.current];
        self.current = (self.current + 1) % self.directions.len();

        Some(res)
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    name: &'a str,
    children: [Weak<Self>; 2],
}

impl<'a> Node<'a> {
    #[inline(always)]
    pub fn get(&self, direction: Direction) -> Weak<Self> {
        self.children[direction as usize].clone()
    }
}

#[derive(Debug, Clone)]
pub struct Map<'a> {
    nodes: Vec<Rc<Node<'a>>>,
    current_node: Weak<Node<'a>>,
    instructions: Instructions,
}

impl<'a> Iterator for Map<'a> {
    type Item = &'a str;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_node = self
            .current_node
            .upgrade()
            .unwrap()
            .get(self.instructions.next().unwrap());

        self.current_node = next_node;
        Some(self.current_node.upgrade().unwrap().name)
    }
}

impl<'a> Map<'a> {
    fn from(value: &'a str, select_origins: impl Fn(&str) -> bool) -> Self {
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

        let mut node_source = HashMap::new();
        let mut origins = Vec::new();

        for line in line_iter.skip(1) {
            let (name, rest) = line.trim().split_once(" = ").unwrap();
            if select_origins(name) {
                origins.push(name);
            }

            if let Ok((_, (left, right))) = parse_moves(rest) {
                node_source.insert(name, (left, right));
            }
        }

        assert!(!origins.is_empty());

        let mut node_index = HashMap::new();

        res.current_node = Self::create_recursive(
            origins.pop().unwrap(),
            &node_source,
            &mut node_index,
            &mut res.nodes,
        );

        for name in origins.iter() {
            res.current_node =
                Self::create_recursive(name, &node_source, &mut node_index, &mut res.nodes);
        }

        res
    }

    fn create_recursive<'b>(
        name: &'a str,
        node_source: &'b HashMap<&'a str, (&'a str, &'a str)>,
        node_index: &mut HashMap<&'a str, Weak<Node<'a>>>,
        created_nodes: &'b mut Vec<Rc<Node<'a>>>,
    ) -> Weak<Node<'a>> {
        if let Some(node) = node_index.get(name) {
            return node.clone();
        }

        let (left, right) = node_source.get(name).unwrap();
        let res = Rc::new_cyclic(|me| {
            node_index.insert(name, me.clone());
            Node {
                name,
                children: [
                    Self::create_recursive(left, node_source, node_index, created_nodes),
                    Self::create_recursive(right, node_source, node_index, created_nodes),
                ],
            }
        });

        created_nodes.push(res);
        Rc::downgrade(created_nodes.last().unwrap())
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
    let map = Map::from(input, |s| s == "AAA");
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
    let map = Map::from(input, |s| s.ends_with('A'));
    let routes = map
        .nodes
        .iter()
        .filter_map(|node| {
            if node.name.ends_with('A') {
                let mut res = map.clone();
                res.current_node = Rc::downgrade(node);
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
