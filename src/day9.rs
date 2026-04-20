use std::fs;
use std::time::Instant;

use anyhow::Result;

const PREAMBLE_LEN: usize = 25;

fn part1(nums: &[u64]) -> u64 {
    for window in nums.windows(PREAMBLE_LEN + 1) {
        let (preamble, &[target]) = window.split_at(PREAMBLE_LEN) else {
            unreachable!()
        };

        let valid = preamble
            .iter()
            .enumerate()
            .any(|(i, &prev)| prev < target && preamble[i + 1..].contains(&(target - prev)));

        if !valid {
            return target;
        }
    }

    unreachable!()
}

fn part2(nums: &[u64], invalid: u64) -> u64 {
    let mut lo = 0;
    let mut hi = 0;
    let mut sum = 0;

    while hi < nums.len() {
        sum += nums[hi];
        hi += 1;

        while sum > invalid {
            sum -= nums[lo];
            lo += 1;
        }

        if sum == invalid && hi - lo >= 2 {
            let range = &nums[lo..hi];
            return range.iter().min().unwrap() + range.iter().max().unwrap();
        }
    }

    unreachable!()
}

fn main() -> Result<()> {
    let nums = fs::read_to_string("in/day9.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let start = Instant::now();
    let part1 = self::part1(&nums);
    let elapsed = start.elapsed();
    println!("Part 1: {part1} ({elapsed:?})");
    assert_eq!(part1, 542_529_149);

    let start = Instant::now();
    let part2 = self::part2(&nums, part1);
    let elapsed = start.elapsed();
    println!("Part 2: {part2} ({elapsed:?})");
    assert_eq!(part2, 75_678_618);

    Ok(())
}
