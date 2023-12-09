use aoc_runner_derive::{aoc, aoc_generator};

type Number = i64;

#[derive(Debug, Default)]
pub struct Sequence {
    numbers: Vec<Number>,
}

impl Sequence {
    fn derive(&self) -> Self {
        Self {
            numbers: self.numbers.windows(2).map(|a| a[1] - a[0]).collect(),
        }
    }

    fn is_zero(&self) -> bool {
        !self.numbers.iter().any(|x| x != &0)
    }

    fn get_next(&self) -> Number {
        if self.is_zero() {
            0
        } else {
            self.numbers.last().unwrap() + self.derive().get_next()
        }
    }

    fn get_previous(&self) -> Number {
        if self.is_zero() {
            0
        } else {
            self.numbers.first().unwrap() - self.derive().get_previous()
        }
    }
}

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Sequence> {
    let mut res = Vec::new();
    for line in input.lines() {
        let mut seq = Sequence::default();
        for num in line.split(' ') {
            seq.numbers.push(num.parse::<Number>().unwrap());
        }
        res.push(seq);
    }
    res
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Sequence]) -> Number {
    input.iter().map(|s| s.get_next()).sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[Sequence]) -> Number {
    input.iter().map(|s| s.get_previous()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 114);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 2);
    }
}
