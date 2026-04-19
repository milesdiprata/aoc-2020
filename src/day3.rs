use std::fs;
use std::iter;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use strum::Display;
use strum::EnumString;

type Pos = aoc_2020::Pos<usize>;

#[derive(Clone, Copy, Debug, Display, EnumString)]
enum Tile {
    #[strum(serialize = ".")]
    Open,
    #[strum(serialize = "#")]
    Tree,
}

struct Map {
    grid: Vec<Tile>,
    height: usize,
    width: usize,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(map: &str) -> Result<Self> {
        let height = map.lines().count();
        let width = map
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty grid"))?
            .len();

        let grid = map
            .lines()
            .flat_map(|row| row.chars())
            .map(|tile| tile.to_string())
            .map(|tile| tile.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

impl Tile {
    const fn is_tree(self) -> bool {
        matches!(self, Self::Tree)
    }
}

impl Map {
    fn get(&self, pos: Pos) -> Option<Tile> {
        if pos.x() < self.width && pos.y() < self.height {
            Some(self.grid[(self.width * pos.y()) + pos.x()])
        } else {
            None
        }
    }

    fn next(&self, pos: Pos, slope: Pos) -> Option<Pos> {
        let next = pos + slope;
        let next = Pos::new(next.x() % self.width, next.y());

        if next.y() < self.height {
            Some(next)
        } else {
            None
        }
    }

    fn traverse(&self, slope: Pos) -> usize {
        iter::successors(Some(Pos::new(0, 0)), |&pos| self.next(pos, slope))
            .filter(|&pos| self.get(pos).is_some_and(Tile::is_tree))
            .count()
    }
}

fn part1(map: &Map) -> usize {
    map.traverse(Pos::new(3, 1))
}

fn part2(map: &Map) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| map.traverse(Pos::new(dx, dy)))
        .product()
}

fn main() -> Result<()> {
    let map = Map::from_str(&fs::read_to_string("in/day3.txt")?)?;

    {
        let start = Instant::now();
        let part1 = part1(&map);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 169);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&map);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 7_560_370_818);
    };

    Ok(())
}
