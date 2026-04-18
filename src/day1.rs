use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

use anyhow::Result;

fn two_sum(report: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut nums = HashSet::new();

    for &num in report {
        let complement = target - num;

        if nums.contains(&complement) {
            return Some((complement, num));
        }

        nums.insert(num);
    }

    None
}

#[allow(dead_code)]
fn three_sum(report: &[i64], target: i64) -> Option<(i64, i64, i64)> {
    for (i, &x) in report.iter().enumerate() {
        if let Some((y, z)) = self::two_sum(&report[i + 1..], target - x) {
            return Some((x, y, z));
        }
    }

    None
}

fn three_sum2(report: &mut [i64], target: i64) -> Option<(i64, i64, i64)> {
    report.sort_unstable();

    for i in 0..report.len() {
        if i > 0 && report[i] == report[i - 1] {
            continue;
        }

        let mut lo = i + 1;
        let mut hi = report.len() - 1;

        while lo < hi {
            let sum = report[i] + report[lo] + report[hi];
            match sum.cmp(&target) {
                Ordering::Less => lo += 1,
                Ordering::Greater => hi -= 1,
                Ordering::Equal => return Some((report[i], report[lo], report[hi])),
            }
        }
    }

    None
}

fn part1(report: &[i64]) -> i64 {
    let (a, b) =
        self::two_sum(report, 2020).unwrap_or_else(|| unreachable!("no 2-sum solution found"));
    a * b
}

fn part2(report: &mut [i64]) -> i64 {
    let (a, b, c) =
        self::three_sum2(report, 2020).unwrap_or_else(|| unreachable!("no 3-sum solution found"));
    a * b * c
}

fn main() -> Result<()> {
    let mut report = fs::read_to_string("in/day1.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&report);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_015_476);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&mut report);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 200_878_544);
    };

    Ok(())
}
