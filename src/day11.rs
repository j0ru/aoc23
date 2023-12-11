use aoc_runner_derive::aoc;
use grid::Grid;
use itertools::Itertools;

type Number = u64;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpaceObject {
    Galaxy,
    Empty,
}

impl From<char> for SpaceObject {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Galaxy,
            '.' => Self::Empty,
            c => panic!("unknown character '{c}'"),
        }
    }
}

#[derive(Debug)]
pub struct StarMap {
    grid: Grid<SpaceObject>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
    factor: usize,
}

impl From<&str> for StarMap {
    fn from(input: &str) -> StarMap {
        let mut data = Vec::new();
        let mut width = 0;
        for line in input.lines() {
            if width == 0 {
                width = line.len();
            }
            data.append(&mut line.chars().map(|c| SpaceObject::from(c)).collect())
        }

        let mut res = StarMap {
            grid: Grid::from_vec(data, width),
            factor: 0,
            empty_rows: Vec::new(),
            empty_cols: Vec::new(),
        };

        for (idx, mut col_iter) in res.grid.iter_cols().enumerate() {
            if col_iter.all(|o| o == &SpaceObject::Empty) {
                res.empty_cols.push(idx);
            }
        }

        for (idx, mut row_iter) in res.grid.iter_rows().enumerate() {
            if row_iter.all(|o| o == &SpaceObject::Empty) {
                res.empty_rows.push(idx);
            }
        }

        println!("{res:?}");

        res
    }
}

impl StarMap {
    pub fn get_galaxies(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for (pos, &object) in self.grid.indexed_iter() {
            if object == SpaceObject::Galaxy {
                let x_offset = (0..=pos.0).fold(0, |acc, x| {
                    if self.empty_rows.contains(&x) {
                        acc + self.factor - 1
                    } else {
                        acc
                    }
                });
                let y_offset = (0..=pos.1).fold(0, |acc, x| {
                    if self.empty_cols.contains(&x) {
                        acc + self.factor - 1
                    } else {
                        acc
                    }
                });
                println!(
                    "{pos:?} + ({x_offset}, {y_offset}) => {:?}",
                    (pos.0 + x_offset, pos.1 + y_offset)
                );
                res.push((pos.0 + x_offset, pos.1 + y_offset));
            }
        }

        res
    }

    pub fn get_unique_routes(&self) -> Vec<((usize, usize), (usize, usize))> {
        self.get_galaxies()
            .iter()
            .permutations(2)
            .unique_by(|x| {
                if x[0].0 > x[1].0 || (x[0].0 == x[1].0 && x[0].1 > x[1].1) {
                    (x[0], x[1])
                } else {
                    (x[1], x[0])
                }
            })
            .map(|x| (x[0].clone(), x[1].clone()))
            .collect()
    }

    pub fn get_distances(&self) -> Number {
        let mut res = 0;
        let routes = self.get_unique_routes();

        for x in routes {
            let from = x.0;
            let to = x.1;
            let distance =
                (from.0 as i32 - to.0 as i32).abs() + (from.1 as i32 - to.1 as i32).abs();

            res += distance as Number
        }

        res
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> Number {
    let mut map = StarMap::from(input);
    map.factor = 2;
    map.get_distances()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> Number {
    let mut map = StarMap::from(input);
    map.factor = 1_000_000;
    map.get_distances()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(solve_part1(input), 374);
    }

    #[test]
    pub fn example_part2() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let mut map = StarMap::from(input);

        map.factor = 10;
        assert_eq!(map.get_distances(), 1030);

        map.factor = 100;
        assert_eq!(map.get_distances(), 8410);
    }
}
