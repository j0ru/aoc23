use aoc_runner_derive::aoc;
use pcre2::bytes::{Captures, Regex};

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut res = 0;
    for line in input.lines() {
        let mut line_res = String::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                line_res.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                line_res.push(c);
                break;
            }
        }
        assert!(!line_res.is_empty());
        res += line_res.parse::<u32>().unwrap();
    }
    res
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|\d))").unwrap();
    let mut res = 0;
    for line in input.lines() {
        let mut line_res = String::new();
        let captures = re.captures_iter(line.as_bytes()).map(|c| c.unwrap());

        let captures: Vec<Captures> = captures.collect();
        for word in [captures.get(0), captures.last()] {
            let word =
                String::from_utf8(word.unwrap().get(1).unwrap().as_bytes().to_vec()).unwrap();
            let num_char = match word.as_str() {
                "one" => '1',
                "two" => '2',
                "three" => '3',
                "four" => '4',
                "five" => '5',
                "six" => '6',
                "seven" => '7',
                "eight" => '8',
                "nine" => '9',
                n => n.chars().next().unwrap(),
            };
            line_res.push(num_char);
        }

        assert!(!line_res.is_empty());
        res += line_res.parse::<u32>().unwrap();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        const EXAMPLE: &'static str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(solve_part1(EXAMPLE), 142);
    }

    #[test]
    fn example_part2() {
        const EXAMPLE: &'static str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(solve_part2(EXAMPLE), 281);
    }
}
