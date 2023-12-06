use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Default)]
pub struct Card {
    winning: Vec<u32>,
    actual: Vec<u32>,
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Card> {
    let re_numbers = Regex::new(r"\d+").unwrap();
    let mut res = Vec::new();
    for line in input.lines() {
        let mut card = Card::default();
        let numbers = line.split(':').last().unwrap();
        if let [winning, actual] = numbers.split('|').collect::<Vec<&str>>().as_slice() {
            for cap in re_numbers.find_iter(winning).map(|c| c.as_str()) {
                card.winning.push(cap.parse::<u32>().unwrap());
            }
            for cap in re_numbers.find_iter(actual).map(|c| c.as_str()) {
                card.actual.push(cap.parse::<u32>().unwrap());
            }
            res.push(card);
        }
    }

    res
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Card]) -> u32 {
    let mut res = 0;
    for card in input {
        let mut card_score = 0;
        for num in card.actual.iter() {
            if card.winning.contains(num) {
                if card_score > 0 {
                    card_score *= 2;
                } else {
                    card_score = 1;
                }
            }
        }
        res += card_score;
    }

    res
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Card]) -> u32 {
    let mut card_pile = vec![1; input.len()];
    for (idx, card) in input.iter().enumerate() {
        let mut card_score = 0;
        for num in card.actual.iter() {
            if card.winning.contains(num) {
                card_score += 1;
            }
        }

        for i in idx + 1..=idx + card_score {
            if i >= card_pile.len() {
                break;
            }

            card_pile[i] += card_pile[idx];
        }
    }

    card_pile.into_iter().reduce(|acc, x| acc + x).unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part1(&input_generator(input)), 13);
    }

    #[test]
    fn example_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(solve_part2(&input_generator(input)), 30);
    }
}
