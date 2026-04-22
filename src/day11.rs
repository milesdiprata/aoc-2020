use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use strum::EnumString;

type Pos = aoc_2020::Pos<usize>;

#[derive(Clone, Copy, EnumString, PartialEq, Eq)]
enum Seat {
    #[strum(serialize = ".")]
    Floor,
    #[strum(serialize = "L")]
    Empty,
    #[strum(serialize = "#")]
    Occupied,
}

#[derive(Clone, PartialEq, Eq)]
struct Layout {
    grid: Vec<Seat>,
    height: usize,
    width: usize,
}

impl FromStr for Layout {
    type Err = Error;

    fn from_str(layout: &str) -> Result<Self> {
        let height = layout.lines().count();
        let width = layout
            .lines()
            .next()
            .ok_or_else(|| anyhow!("empty layout"))?
            .len();

        let grid = layout
            .lines()
            .flat_map(|line| line.chars())
            .map(|tile| tile.to_string().parse())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            grid,
            height,
            width,
        })
    }
}

impl Seat {
    const fn is_floor(self) -> bool {
        matches!(self, Self::Floor)
    }

    const fn is_occupied(self) -> bool {
        matches!(self, Self::Occupied)
    }
}

impl Layout {
    const DIRS: [fn(Pos) -> Option<Pos>; 8] = [
        |pos: Pos| pos.up(),
        |pos: Pos| pos.up().and_then(Pos::right),
        |pos: Pos| pos.right(),
        |pos: Pos| pos.down().and_then(Pos::right),
        |pos: Pos| pos.down(),
        |pos: Pos| pos.down().and_then(Pos::left),
        |pos: Pos| pos.left(),
        |pos: Pos| pos.up().and_then(Pos::left),
    ];

    fn adj(pos: Pos) -> impl Iterator<Item = Pos> {
        Self::DIRS.into_iter().filter_map(move |dir| dir(pos))
    }

    fn visible(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        Self::DIRS.into_iter().filter_map(move |dir| {
            let mut pos = dir(pos);
            while pos.is_some_and(|pos| self.get(pos).is_some_and(Seat::is_floor)) {
                pos = pos.and_then(dir);
            }
            pos
        })
    }

    fn iter(&self) -> impl Iterator<Item = Pos> {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| Pos::new(x, y)))
    }

    fn get(&self, pos: Pos) -> Option<Seat> {
        if pos.x() < self.width && pos.y() < self.height {
            Some(self.grid[(self.width * pos.y()) + pos.x()])
        } else {
            None
        }
    }

    fn seat(&mut self, count: fn(&Self, Pos) -> usize, threshold: usize) -> bool {
        let next = self
            .iter()
            .map(|pos| {
                let occupied = count(self, pos);
                match self.get(pos).unwrap() {
                    Seat::Empty if occupied == 0 => Seat::Occupied,
                    Seat::Occupied if occupied >= threshold => Seat::Empty,
                    seat => seat,
                }
            })
            .collect();

        let changed = next != self.grid;
        self.grid = next;
        changed
    }
}

fn simulate(mut layout: Layout, count: fn(&Layout, Pos) -> usize, threshold: usize) -> usize {
    while layout.seat(count, threshold) {}
    layout
        .iter()
        .filter(|&pos| layout.get(pos).is_some_and(Seat::is_occupied))
        .count()
}

fn part1(layout: Layout) -> usize {
    self::simulate(
        layout,
        |layout, pos| {
            Layout::adj(pos)
                .filter(|&pos| layout.get(pos).is_some_and(Seat::is_occupied))
                .count()
        },
        4,
    )
}

fn part2(layout: Layout) -> usize {
    self::simulate(
        layout,
        |layout, pos| {
            layout
                .visible(pos)
                .filter(|&pos| layout.get(pos).is_some_and(Seat::is_occupied))
                .count()
        },
        5,
    )
}

fn main() -> Result<()> {
    let layout = Layout::from_str(&fs::read_to_string("in/day11.txt")?)?;

    {
        let layout = layout.clone();
        let start = Instant::now();
        let part1 = self::part1(layout);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 2_254);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(layout);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 2_004);
    };

    Ok(())
}
