use aoc_runner_derive::{aoc, aoc_generator};
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, ops::Range};

#[derive(Debug, Default)]
pub struct ResourceMap {
    source_ranges: Vec<Range<u64>>,
    destination_ranges: Vec<Range<u64>>,
}

impl ResourceMap {
    fn map_to_destination(&self, source: u64) -> u64 {
        for (idx, range) in self.source_ranges.iter().enumerate() {
            if range.contains(&source) {
                return self.destination_ranges[idx].start + (source - range.start);
            }
        }
        source
    }

    fn insert_range(&mut self, source_start: u64, dest_start: u64, len: u64) {
        self.source_ranges.push(source_start..source_start + len);
        self.destination_ranges.push(dest_start..dest_start + len);
    }
}

#[derive(Debug, Default)]
pub struct Maps {
    maps: HashMap<(String, String), ResourceMap>,
    seeds: Vec<u64>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Maps {
    let mut res = Maps::default();
    let re_source_destination = Regex::new(r"(?<source>\w+)-to-(?<destination>\w+)\smap:").unwrap();
    let re_range = Regex::new(r"(?<destination>\d+)\s(?<source>\d+)\s(?<range_len>\d+)").unwrap();
    let re_seeds = Regex::new(r"\d+").unwrap();

    let mut line_iter = input.lines();
    for seed_match in re_seeds
        .find_iter(line_iter.next().unwrap())
        .map(|c| c.as_str())
    {
        res.seeds.push(seed_match.parse::<u64>().unwrap());
    }

    let mut current_src_dest = Some((String::new(), String::new()));
    while let Some(line) = line_iter.next() {
        if let Some(caps) = re_source_destination.captures(line) {
            current_src_dest.replace((caps["source"].to_string(), caps["destination"].to_string()));
        } else if let Some(caps) = re_range.captures(line) {
            let key = current_src_dest.as_ref().unwrap();
            let entry = res.maps.entry(key.clone()).or_default();
            entry.insert_range(
                caps["source"].parse::<u64>().unwrap(),
                caps["destination"].parse::<u64>().unwrap(),
                caps["range_len"].parse::<u64>().unwrap(),
            );
        }
    }

    res
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Maps) -> u64 {
    let mut res = u64::MAX;
    let foo = [
        String::from("seed"),
        String::from("soil"),
        String::from("fertilizer"),
        String::from("water"),
        String::from("light"),
        String::from("temperature"),
        String::from("humidity"),
        String::from("location"),
    ];
    for seed in input.seeds.iter() {
        let mut latest = *seed;
        for mapping in foo.windows(2) {
            latest = input
                .maps
                .get(&(mapping[0].clone(), mapping[1].clone()))
                .unwrap()
                .map_to_destination(latest);
        }
        res = res.min(latest);
    }

    res
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Maps) -> u64 {
    let mut res = u64::MAX;
    let foo = [
        String::from("seed"),
        String::from("soil"),
        String::from("fertilizer"),
        String::from("water"),
        String::from("light"),
        String::from("temperature"),
        String::from("humidity"),
        String::from("location"),
    ];
    for seed_range in input.seeds.chunks(2) {
        let range = seed_range[0]..seed_range[0] + seed_range[1];
        println!("working on range: {range:?}");
        res = res.min(
            range
                .into_par_iter()
                .map(|seed| {
                    let mut latest = seed;
                    for mapping in foo.windows(2) {
                        latest = input
                            .maps
                            .get(&(mapping[0].clone(), mapping[1].clone()))
                            .unwrap()
                            .map_to_destination(latest);
                    }
                    latest
                })
                .min()
                .unwrap(),
        );
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    pub fn example_part1() {
        assert_eq!(solve_part1(&input_generator(EXAMPLE)), 35);
    }

    #[test]
    pub fn example_part2() {
        assert_eq!(solve_part2(&input_generator(EXAMPLE)), 46);
    }
}
