use std::{cmp::max, collections::HashMap};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Round {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Round {
    pub fn possible(&self, maximums: (u32, u32, u32)) -> bool {
        self.reds <= maximums.0 && self.greens <= maximums.1 && self.blues <= maximums.2
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let mut sp = l.split(':');
            let id = sp
                .next()
                .unwrap()
                .split(' ')
                .nth(1)
                .map(|n| n.parse::<u32>().unwrap())
                .unwrap();
            let rounds = sp
                .next()
                .unwrap()
                .split(';')
                .map(|raw| {
                    let mut res = HashMap::new();
                    raw.split(',').for_each(|pull| {
                        let mut sp = pull.trim().split(' ');
                        let num = sp.next().unwrap().parse::<u32>().unwrap();
                        let color = sp.next().unwrap().to_string();
                        res.insert(color, num);
                    });
                    Round {
                        reds: *res.get("red").unwrap_or(&0),
                        greens: *res.get("green").unwrap_or(&0),
                        blues: *res.get("blue").unwrap_or(&0),
                    }
                })
                .collect();
            Game { id, rounds }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    const MAX: (u32, u32, u32) = (12, 13, 14);

    input
        .iter()
        .map(|g| {
            if g.rounds.iter().any(|r| !r.possible(MAX)) {
                0
            } else {
                g.id
            }
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| {
            let res = g.rounds.iter().fold(Round::default(), |acc, x| Round {
                reds: max(acc.reds, x.reds),
                greens: max(acc.greens, x.greens),
                blues: max(acc.blues, x.blues),
            });

            res.reds * res.greens * res.blues
        })
        .reduce(|a, b| a + b)
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn convert_input() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

        let res = input_generator(INPUT);
        let correct_res = vec![
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        blues: 3,
                        reds: 4,
                        greens: 0,
                    },
                    Round {
                        reds: 1,
                        blues: 6,
                        greens: 2,
                    },
                    Round {
                        greens: 2,
                        ..Default::default()
                    },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    Round {
                        blues: 1,
                        reds: 0,
                        greens: 2,
                    },
                    Round {
                        reds: 1,
                        blues: 4,
                        greens: 3,
                    },
                    Round {
                        greens: 1,
                        blues: 1,
                        ..Default::default()
                    },
                ],
            },
        ];
        assert_eq!(res, correct_res);
    }

    #[test]
    pub fn example_part1() {
        const EXAMPLE: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, solve_part1(&input_generator(EXAMPLE)));
    }

    #[test]
    pub fn example_part2() {
        const EXAMPLE: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, solve_part2(&input_generator(EXAMPLE)));
    }
}
