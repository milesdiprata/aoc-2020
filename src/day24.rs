use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use strum::EnumString;

#[derive(Clone, Copy, Debug, EnumString)]
enum Dir {
    #[strum(serialize = "e")]
    East,
    #[strum(serialize = "se")]
    Southeast,
    #[strum(serialize = "sw")]
    Southwest,
    #[strum(serialize = "w")]
    West,
    #[strum(serialize = "nw")]
    Northwest,
    #[strum(serialize = "ne")]
    Northeast,
}

type Pos = aoc_2020::Pos<i32>;

#[derive(Debug)]
struct Tile {
    dirs: Vec<Dir>,
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(dirs: &str) -> Result<Self> {
        let mut tile = Self { dirs: Vec::new() };
        let mut i = 0;

        while i < dirs.len() {
            if i + 1 < dirs.len()
                && let Ok(dir) = dirs[i..i + 2].parse()
            {
                tile.dirs.push(dir);
                i += 2;
            } else {
                tile.dirs.push(dirs[i..=i].parse()?);
                i += 1;
            }
        }

        Ok(tile)
    }
}

impl Dir {
    fn iter() -> impl Iterator<Item = Self> {
        [
            Self::East,
            Self::Southeast,
            Self::Southwest,
            Self::West,
            Self::Northwest,
            Self::Northeast,
        ]
        .into_iter()
    }

    const fn step(self, pos: Pos) -> Pos {
        match self {
            Self::East => pos.right(),
            Self::Southeast => pos.down(),
            Self::Southwest => pos.down().left(),
            Self::West => pos.left(),
            Self::Northwest => pos.up(),
            Self::Northeast => pos.up().right(),
        }
    }
}

impl Tile {
    fn coord(&self) -> Pos {
        self.dirs
            .iter()
            .fold(Pos::new(0, 0), |pos, &dir| dir.step(pos))
    }
}

fn part1(tiles: &[Tile]) -> HashSet<Pos> {
    let mut black = HashSet::new();
    for tile in tiles {
        let coord = tile.coord();
        if !black.insert(coord) {
            black.remove(&coord);
        }
    }

    black
}

fn part2(black: HashSet<Pos>) -> usize {
    fn neighbors(pos: Pos) -> impl Iterator<Item = Pos> {
        Dir::iter().map(move |dir| dir.step(pos))
    }

    fn step(black: &HashSet<Pos>) -> HashSet<Pos> {
        let mut counts = HashMap::new();
        for &pos in black {
            for adj in neighbors(pos) {
                *counts.entry(adj).or_insert(0_usize) += 1;
            }
        }

        counts
            .into_iter()
            .filter(|&(pos, count)| {
                if black.contains(&pos) {
                    count == 1 || count == 2
                } else {
                    count == 2
                }
            })
            .map(|(pos, _)| pos)
            .collect()
    }

    (0..100).fold(black, |black, _| step(&black)).len()
}

fn main() -> Result<()> {
    let tiles = fs::read_to_string("in/day24.txt")?
        .lines()
        .map(Tile::from_str)
        .collect::<Result<Vec<_>>>()?;

    let start = Instant::now();
    let black = self::part1(&tiles);
    let part1 = black.len();
    let elapsed = start.elapsed();
    println!("Part 1: {part1} ({elapsed:?})");
    assert_eq!(part1, 244);

    let start = Instant::now();
    let part2 = self::part2(black);
    let elapsed = start.elapsed();
    println!("Part 2: {part2} ({elapsed:?})");
    assert_eq!(part2, 3_665);

    Ok(())
}
