use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

struct Group {
    answers_any: u32,
    answers_all: u32,
}

impl FromStr for Group {
    type Err = Error;

    fn from_str(group: &str) -> Result<Self> {
        fn person_mask(line: &str) -> Result<u32> {
            line.chars().try_fold(0, |mask, c| {
                if c.is_ascii_lowercase() {
                    Ok(mask | (1 << (c as u8 - b'a')))
                } else {
                    bail!("invalid question '{c}'")
                }
            })
        }

        let people = group.lines().map(person_mask).collect::<Result<Vec<_>>>()?;

        Ok(Self {
            answers_any: people.iter().fold(0, |a, &b| a | b),
            answers_all: people.iter().copied().reduce(|a, b| a & b).unwrap_or(0),
        })
    }
}

fn part1(groups: &[Group]) -> u32 {
    groups
        .iter()
        .map(|group| group.answers_any.count_ones())
        .sum()
}

fn part2(groups: &[Group]) -> u32 {
    groups
        .iter()
        .map(|group| group.answers_all.count_ones())
        .sum()
}

fn main() -> Result<()> {
    let groups = fs::read_to_string("in/day6.txt")?
        .split("\n\n")
        .map(Group::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&groups);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 6_686);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&groups);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 3_476);
    };

    Ok(())
}
