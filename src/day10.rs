use std::fs;
use std::time::Instant;

use anyhow::Result;

fn make_chain(joltages: &[u32]) -> Vec<u32> {
    const OUTLET: u32 = 0;

    let device = joltages.iter().max().unwrap() + 3;

    let mut chain = joltages
        .iter()
        .copied()
        .chain([OUTLET, device])
        .collect::<Vec<_>>();
    chain.sort_unstable();

    chain
}

fn part1(joltages: &[u32]) -> usize {
    let chain = self::make_chain(joltages);

    let mut ones = 0;
    let mut threes = 0;

    for window in chain.windows(2) {
        match window[1] - window[0] {
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }

    ones * threes
}

fn part2(joltages: &[u32]) -> usize {
    let chain = self::make_chain(joltages);
    let device = chain.last().copied().unwrap() as usize;

    let mut joltages = vec![false; device + 1];
    for &joltage in &chain {
        joltages[joltage as usize] = true;
    }

    let mut ways = vec![0; device + 1];
    ways[0] = 1;

    for &joltage in &chain[1..] {
        for delta in [1, 2, 3] {
            let Some(prev) = joltage.checked_sub(delta).map(|prev| prev as usize) else {
                continue;
            };

            if joltages[prev] {
                ways[joltage as usize] += ways[prev];
            }
        }
    }

    ways[device]
}

fn main() -> Result<()> {
    let joltages = fs::read_to_string("in/day10.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&joltages);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 2_738);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&joltages);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 74_049_191_673_856);
    };

    Ok(())
}
