use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BoardingPass {
    seat: u16,
}

impl FromStr for BoardingPass {
    type Err = Error;

    fn from_str(pass: &str) -> Result<Self> {
        // ID = (8 * row) + col
        let seat = pass.chars().try_fold(0, |acc, c| match c {
            'F' | 'L' => Ok(acc << 1),
            'B' | 'R' => Ok((acc << 1) | 1),
            _ => Err(anyhow!("invalid partition '{c}'")),
        })?;

        Ok(Self { seat })
    }
}

fn part1(passes: &[BoardingPass]) -> u16 {
    passes
        .iter()
        .map(|&pass| pass.seat)
        .max()
        .unwrap_or_default()
}

fn part2(passes: &mut [BoardingPass]) -> u16 {
    passes.sort_unstable();
    passes
        .windows(2)
        .find(|&window| window[1].seat - window[0].seat == 2)
        .map_or_else(
            || unreachable!("no gap in seats exist"),
            |window| window[0].seat + 1,
        )
}

fn main() -> Result<()> {
    let mut passes = fs::read_to_string("in/day5.txt")?
        .lines()
        .map(BoardingPass::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&passes);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 848);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&mut passes);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 682);
    };

    Ok(())
}
