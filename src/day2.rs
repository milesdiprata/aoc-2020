use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Policy {
    letter: char,
    occurrences: RangeInclusive<usize>,
}

#[derive(Debug)]
struct Password {
    policy: Policy,
    pass: String,
}

impl FromStr for Policy {
    type Err = Error;

    fn from_str(policy: &str) -> Result<Self> {
        let (range, letter) = policy
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid policy '{policy}'"))?;
        let (lo, hi) = range
            .split_once('-')
            .ok_or_else(|| anyhow!("invalid range '{range}'"))?;

        Ok(Self {
            letter: letter
                .chars()
                .next()
                .ok_or_else(|| anyhow!("invalid letter '{letter}'"))?,
            occurrences: lo.parse()?..=hi.parse()?,
        })
    }
}

impl FromStr for Password {
    type Err = Error;

    fn from_str(pass: &str) -> Result<Self> {
        let (policy, pass) = pass
            .split_once(": ")
            .ok_or_else(|| anyhow!("invalid password '{pass}'"))?;

        Ok(Self {
            policy: policy.parse()?,
            pass: pass.to_string(),
        })
    }
}

impl Password {
    fn is_valid(&self) -> bool {
        let count = self
            .pass
            .chars()
            .filter(|&letter| letter == self.policy.letter)
            .count();

        self.policy.occurrences.contains(&count)
    }

    fn is_valid2(&self) -> bool {
        let i = *self.policy.occurrences.start() - 1;
        let j = *self.policy.occurrences.end() - 1;
        let pass = self.pass.as_bytes();

        [pass[i], pass[j]]
            .into_iter()
            .filter(|&letter| letter == self.policy.letter as u8)
            .count()
            == 1
    }
}

fn part1(passes: &[Password]) -> usize {
    passes.iter().filter(|&pass| pass.is_valid()).count()
}

fn part2(passes: &[Password]) -> usize {
    passes.iter().filter(|&pass| pass.is_valid2()).count()
}

fn main() -> Result<()> {
    let passes = fs::read_to_string("in/day2.txt")?
        .lines()
        .map(Password::from_str)
        .collect::<Result<Vec<_>>>()?;

    dbg!(&passes);

    {
        let start = Instant::now();
        let part1 = self::part1(&passes);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 378);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&passes);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 280);
    };

    Ok(())
}
