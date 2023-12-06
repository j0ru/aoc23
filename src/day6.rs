use aoc_runner_derive::aoc;
use regex::Regex;

type Number = u64;

#[derive(Debug)]
pub struct Race {
    time: Number,
    distance: Number,
}

fn fetch_numbers(input: &str) -> Vec<Number> {
    let re_numbers = Regex::new(r"\d+").unwrap();

    re_numbers
        .find_iter(input)
        .map(|c| c.as_str().parse::<Number>().unwrap())
        .collect()
}

fn races_from_str(input: &str) -> Vec<Race> {
    let mut line_iter = input.lines();
    let times = fetch_numbers(line_iter.next().unwrap());
    let distances = fetch_numbers(line_iter.next().unwrap());

    assert_eq!(times.len(), distances.len());

    times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect()
}

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> Number {
    let input = races_from_str(input);
    let mut res = 1;

    for race in input.iter() {
        res *= (1..race.time)
            .map(|i| (race.time - i) * i)
            .fold(0, |acc, x| if x > race.distance { acc + 1 } else { acc });
    }
    res
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> Number {
    solve_part1(&input.replace(' ', ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(EXAMPLE), 288);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(EXAMPLE), 71503);
    }
}
