use std::collections::HashSet;

use aoc_runner_derive::aoc;

type Number = u32;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn advance(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Pos {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Pos {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Self::South,
            Direction::East => Self::West,
            Direction::South => Self::North,
            Direction::West => Self::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Pipe([Direction; 2]),
    Origin,
    Ground,
}

impl Tile {
    fn has_direction(&self, dir: Direction) -> bool {
        if self == &Self::Origin {
            true
        } else if let Self::Pipe(directions) = self {
            directions.contains(&dir)
        } else {
            false
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        use Direction::*;
        match value {
            '|' => Self::Pipe([North, South]),
            '-' => Self::Pipe([East, West]),
            'L' => Self::Pipe([North, East]),
            'J' => Self::Pipe([North, West]),
            '7' => Self::Pipe([South, West]),
            'F' => Self::Pipe([South, East]),
            '.' => Self::Ground,
            'S' => Self::Origin,
            c => panic!("unexpected character {c}"),
        }
    }
}

#[derive(Default, Debug)]
pub struct Map {
    width: usize,
    grid: Vec<Tile>,
    origin: Pos,
}

impl Map {
    pub fn get(&self, pos: Pos) -> Option<&Tile> {
        if self.in_bounds(&pos) {
            self.grid.get((pos.y * self.width as i32 + pos.x) as usize)
        } else {
            None
        }
    }

    pub fn walk(&self, from: Pos, dir: Direction) -> Option<(Pos, Tile)> {
        let from_tile = self.get(from)?;
        let next_tile = self.get(from.advance(dir))?;

        if from_tile.has_direction(dir) && next_tile.has_direction(dir.opposite()) {
            Some((from.advance(dir), *next_tile))
        } else {
            None
        }
    }

    pub fn in_bounds(&self, pos: &Pos) -> bool {
        pos.x >= 0
            && pos.x < self.width as i32
            && pos.y >= 0
            && pos.y < (self.grid.len() / self.width) as i32
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut res = Map::default();

        for line in value.lines() {
            if line.len() > res.width {
                res.width = line.len();
            }

            let mut tiles = line.chars().map(|c| Tile::from(c)).collect();
            res.grid.append(&mut tiles);
        }

        for (idx, tile) in res.grid.iter().enumerate() {
            if tile == &Tile::Origin {
                res.origin = Pos {
                    x: (idx % res.width) as i32,
                    y: (idx / res.width) as i32,
                };
                break;
            }
        }

        res
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> Number {
    use Direction::*;
    let map = Map::from(input);

    let mut res = 0;
    for dir in [North, South, West, East] {
        let mut dir_res = 0;
        let mut current = map.origin;
        let mut current_direction = dir;
        while let Some((pos, tile)) = map.walk(current, current_direction) {
            dir_res += 1;
            match tile {
                Tile::Ground => unreachable!("How did you end up on the ground? O.o"),
                Tile::Origin => {
                    res = res.max(dir_res / 2);
                    break;
                }
                Tile::Pipe(ends) => {
                    current = pos;
                    if ends[0] == current_direction.opposite() {
                        current_direction = ends[1]
                    } else {
                        current_direction = ends[0]
                    }
                }
            }
        }
    }

    res
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> Number {
    use Direction::*;
    let map = Map::from(input);
    let mut loop_tiles = HashSet::from([map.origin]);

    'outer: for dir in [North, South, West, East] {
        let mut current = map.origin;
        let mut current_direction = dir;
        while let Some((pos, tile)) = map.walk(current, current_direction) {
            match tile {
                Tile::Ground => unreachable!("How did you end up on the ground? O.o"),
                Tile::Origin => {
                    break 'outer;
                }
                Tile::Pipe(ends) => {
                    loop_tiles.insert(pos);
                    current = pos;
                    if ends[0] == current_direction.opposite() {
                        current_direction = ends[1]
                    } else {
                        current_direction = ends[0]
                    }
                }
            }
        }
    }

    let mut marked = HashSet::new();
    discover_neighbours(Pos { x: 0, y: 0 }, &map, &mut marked, &loop_tiles);
    let non_loop_tiles = count_non_loop_tiles(&marked, &loop_tiles, &map);
    map.grid.len() as u32 - non_loop_tiles - loop_tiles.len() as u32
}

fn discover_neighbours(node: Pos, map: &Map, marked: &mut HashSet<Pos>, loop_tiles: &HashSet<Pos>) {
    use Direction::*;
    if marked.contains(&node) {
        return;
    } else {
        marked.insert(node);
    }
    for dir in [North, South, West, East] {
        let (from, dir_cross) = match dir {
            North => (node, East),
            South => (node.advance(South), East),
            East => (node.advance(East), South),
            West => (node, South),
        };

        if node.x >= -1
            && node.y >= -1
            && node.x <= map.width as i32
            && node.y <= (map.grid.len() / map.width) as i32
        {
            if loop_tiles.contains(&from) && loop_tiles.contains(&from.advance(dir_cross)) {
                if matches!(map.walk(from, dir_cross), Some(_)) {
                    continue;
                }
            }

            discover_neighbours(node.advance(dir), map, marked, loop_tiles);
        }
    }
}

fn count_non_loop_tiles(marked: &HashSet<Pos>, loop_tiles: &HashSet<Pos>, map: &Map) -> Number {
    let mut found_tiles = HashSet::new();
    for knot in marked {
        for x in 0..=1 {
            for y in 0..=1 {
                let to_check = Pos {
                    x: knot.x + x,
                    y: knot.y + y,
                };
                if let Some(_) = map.get(to_check) {
                    if !loop_tiles.contains(&to_check) {
                        found_tiles.insert(to_check);
                    }
                }
            }
        }
    }
    found_tiles.len() as Number
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn example_part1() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(solve_part1(input), 8);
    }

    #[test]
    pub fn example_part2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(solve_part2(input), 10);
    }

    #[test]
    pub fn example_part2_2() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(solve_part2(input), 4);
    }
}
