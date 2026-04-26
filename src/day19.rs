use std::fs;
use std::time::Instant;

use anyhow::Result;

fn part1() -> u64 {
    todo!()
}

fn part2() -> u64 {
    todo!()
}

fn main() -> Result<()> {
    let _input = fs::read_to_string("in/day19.txt")?;

    {
        let start = Instant::now();
        let part1 = self::part1();
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 0);
    };

    {
        let start = Instant::now();
        let part2 = self::part2();
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 0);
    };

    Ok(())
}
