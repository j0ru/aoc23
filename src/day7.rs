use std::{cmp::Ordering, collections::HashMap, ops::Add};

use aoc_runner_derive::{aoc, aoc_generator};

type Number = u32;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Card {
    value: u8,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value = match value {
            'J' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            '9' => 8,
            'T' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => return Err("Unrecognized value".into()),
        };

        Ok(Card { value })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum HandType {
    FiveOfAKind(Card),
    FourOfAKind { member: Card, rest: Card },
    FullHouse { three: Card, two: Card },
    ThreeOfAKind { member: Card, rest: [Card; 2] },
    TwoPair { pairs: [Card; 2], rest: Card },
    OnePair { pair: Card, rest: [Card; 3] },
    HighCard([Card; 5]),
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        use HandType::*;
        let to_num = |t| match t {
            &FiveOfAKind(_) => 6,
            &FourOfAKind { .. } => 5,
            &FullHouse { .. } => 4,
            &ThreeOfAKind { .. } => 3,
            &TwoPair { .. } => 2,
            &OnePair { .. } => 1,
            &HighCard(_) => 0,
        };
        to_num(self).cmp(&to_num(other))
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&Hand> for HandType {
    fn from(value: &Hand) -> Self {
        let mut card_map: HashMap<Card, u8> = HashMap::new();
        for card in value.cards.iter() {
            let entry = card_map.entry(*card).or_insert(0);
            *entry = entry.add(1);
        }

        if card_map.len() > 1 {
            if let Some(num_jokers) = card_map.remove(&Card { value: 0 }) {
                let most_of_in_hand = {
                    let mut card_iter = card_map.iter();
                    let mut most_cards = card_iter.next().unwrap();
                    while let Some((card, num)) = card_iter.next() {
                        if num > most_cards.1 {
                            most_cards = (card, num);
                        }
                    }
                    most_cards.0.clone()
                };
                let most_cards = card_map.get_mut(&most_of_in_hand).unwrap();
                *most_cards += num_jokers;
            }
        }

        if card_map.len() == 1 {
            HandType::FiveOfAKind(card_map.keys().next().unwrap().clone())
        } else if card_map.len() == 2 {
            let mut card_iter = card_map.into_iter();
            match card_iter.next().unwrap() {
                (card, 1) => HandType::FourOfAKind {
                    member: card_iter.next().unwrap().0,
                    rest: card,
                },
                (card, 2) => HandType::FullHouse {
                    three: card_iter.next().unwrap().0,
                    two: card,
                },
                (card, 3) => HandType::FullHouse {
                    three: card,
                    two: card_iter.next().unwrap().0,
                },
                (card, 4) => HandType::FourOfAKind {
                    member: card,
                    rest: card_iter.next().unwrap().0,
                },
                _ => unreachable!(),
            }
        } else if card_map.len() == 3 {
            let mut cards: Vec<(Card, u8)> = card_map.into_iter().collect();
            cards.sort_by(|a, b| b.1.cmp(&a.1));
            if cards[0].1 == 3 {
                HandType::ThreeOfAKind {
                    member: cards[0].0,
                    rest: [cards[1].0, cards[2].0],
                }
            } else {
                HandType::TwoPair {
                    pairs: [cards[0].0, cards[1].0],
                    rest: cards[2].0,
                }
            }
        } else if card_map.len() == 4 {
            let mut cards: Vec<(Card, u8)> = card_map.into_iter().collect();
            cards.sort_by(|a, b| b.1.cmp(&a.1));
            HandType::OnePair {
                pair: cards[0].0,
                rest: [cards[1].0, cards[2].0, cards[3].0],
            }
        } else {
            let cards: Vec<Card> = card_map.into_iter().map(|(c, _)| c).collect();
            HandType::HighCard(cards.try_into().unwrap())
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    cards: [Card; 5],
    bid: Number,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (self_type, other_type) = (HandType::from(self), HandType::from(other));
        match self_type.cmp(&other_type) {
            Ordering::Equal => {
                let mut self_cards = self.cards.iter();
                let mut other_cards = other.cards.iter();
                loop {
                    let (a, b) = (self_cards.next().unwrap(), other_cards.next().unwrap());
                    match a.value.cmp(&b.value) {
                        Ordering::Equal => continue,
                        o => break o,
                    }
                }
            }
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Hand> {
    let mut res = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        let mut sp = line.split(' ');
        let cards: Vec<Card> = sp
            .next()
            .unwrap()
            .chars()
            .map(|c| Card::try_from(c).unwrap())
            .collect();
        let bid = sp.next().unwrap().parse::<Number>().unwrap();
        res.push(Hand {
            cards: cards.try_into().unwrap(),
            bid,
        })
    }

    res
}

#[aoc(day7, part1)]
fn solve_part1(input: &[Hand]) -> Number {
    let mut hands = input.to_vec();
    hands.sort();

    let mut res = 0;
    for (idx, hand) in hands.iter().enumerate() {
        res += hand.bid * (idx as Number + 1);
    }

    res
}

#[aoc(day7, part2)]
fn solve_part2(input: &[Hand]) -> Number {
    let mut hands = input.to_vec();
    hands.sort();

    let mut res = 0;
    for (idx, hand) in hands.iter().enumerate() {
        res += hand.bid * (idx as Number + 1);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn example_part1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(solve_part1(&input_generator(input)), 6440);
    }

    #[test]
    fn example_part2() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut hands = input_generator(input);
        hands.sort();

        assert_eq!(solve_part1(&input_generator(input)), 5905);
    }
}
