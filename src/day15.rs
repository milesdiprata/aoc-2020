use std::fs;
use std::time::Instant;

use anyhow::Result;

#[allow(clippy::cast_possible_truncation)]
fn play(nums: &[u64], target: usize) -> u64 {
    let mut last_turn = vec![0; target];
    for (i, &num) in nums[..nums.len() - 1].iter().enumerate() {
        last_turn[num as usize] = i + 1;
    }

    let mut last = *nums.last().unwrap();
    for turn in nums.len()..target {
        // let next = last_turn.insert(last, turn).map_or(0, |prev| turn - prev);
        let next = if last_turn[last as usize] == 0 {
            0
        } else {
            turn - last_turn[last as usize]
        };

        last_turn[last as usize] = turn;
        last = next as u64;
    }

    last
}

fn main() -> Result<()> {
    let nums = fs::read_to_string("in/day15.txt")?
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::play(&nums, 2020);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1111);
    };

    {
        let start = Instant::now();
        let part2 = self::play(&nums, 30_000_000);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 48_568);
    };

    Ok(())
}
