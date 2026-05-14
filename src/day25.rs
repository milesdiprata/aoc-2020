use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Copy, Debug)]
struct PublicKey(u64);

impl FromStr for PublicKey {
    type Err = Error;

    fn from_str(key: &str) -> Result<Self> {
        Ok(Self(key.parse()?))
    }
}

impl PublicKey {
    fn find_loop_size(self, subject: u64) -> usize {
        let mut val = 1;
        let mut loop_size = 0;

        while val != self.0 {
            val = self::transform(val, subject, 1);
            loop_size += 1;
        }

        loop_size
    }
}

fn transform(mut val: u64, subject: u64, loop_size: usize) -> u64 {
    for _ in 0..loop_size {
        val *= subject;
        val %= 20_201_227;
    }
    val
}

fn parse() -> Result<(PublicKey, PublicKey)> {
    let input = fs::read_to_string("in/day25.txt")?;
    let (card, door) = input
        .split_once('\n')
        .ok_or_else(|| anyhow!("invalid input"))?;

    Ok((card.parse()?, door.parse()?))
}

fn part1(card: PublicKey, door: PublicKey) -> u64 {
    let card_loop_size = card.find_loop_size(7);
    self::transform(1, door.0, card_loop_size)
}

fn main() -> Result<()> {
    let (card, door) = self::parse()?;

    let start = Instant::now();
    let part1 = self::part1(card, door);
    let elapsed = start.elapsed();

    println!("Part 1: {part1} ({elapsed:?})");
    assert_eq!(part1, 448_851);

    Ok(())
}
