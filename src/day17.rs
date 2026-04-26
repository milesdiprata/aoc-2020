use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;

#[derive(Clone, Debug)]
struct PocketDimension<const N: usize> {
    active: HashSet<[i64; N]>,
}

impl<const N: usize> FromStr for PocketDimension<N> {
    type Err = Error;

    fn from_str(cubes: &str) -> Result<Self> {
        let mut active = HashSet::new();
        for (y, row) in cubes.lines().enumerate() {
            for (x, state) in row.char_indices() {
                if state == '#' {
                    let mut pos = [0; N];
                    #[allow(clippy::cast_possible_wrap)]
                    {
                        pos[0] = x as i64;
                        pos[1] = y as i64;
                    }
                    active.insert(pos);
                }
            }
        }

        Ok(Self { active })
    }
}

fn neighbors<const N: usize>(pos: [i64; N]) -> impl Iterator<Item = [i64; N]> {
    #[allow(clippy::cast_possible_truncation)]
    (0..3i64.pow(N as u32)).filter_map(move |mut k| {
        let mut out = pos;
        let mut nonzero = false;

        for slot in &mut out {
            let d = (k % 3) - 1;
            k /= 3;
            *slot += d;
            nonzero |= d != 0;
        }

        nonzero.then_some(out)
    })
}

impl<const N: usize> PocketDimension<N> {
    fn cycle(&mut self) {
        let mut counts = HashMap::new();
        for &pos in &self.active {
            for adj in neighbors(pos) {
                *counts.entry(adj).or_insert(0_usize) += 1;
            }
        }

        self.active = counts
            .into_iter()
            .filter_map(|(pos, count)| match count {
                3 => Some(pos),
                2 if self.active.contains(&pos) => Some(pos),
                _ => None,
            })
            .collect();
    }
}

fn simulate<const N: usize>(mut dim: PocketDimension<N>) -> usize {
    for _ in 0..6 {
        dim.cycle();
    }

    dim.active.len()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("in/day17.txt")?;
    let dim3d = input.parse::<PocketDimension<3>>()?;
    let dim4d = input.parse::<PocketDimension<4>>()?;

    {
        let start = Instant::now();
        let part1 = self::simulate(dim3d);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 448);
    };

    {
        let start = Instant::now();
        let part2 = self::simulate(dim4d);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 2_400);
    };

    Ok(())
}
