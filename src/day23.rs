use std::fs;
use std::time::Instant;

use anyhow::Result;
use anyhow::anyhow;

fn part1(mut cups: Vec<u8>) -> u32 {
    let min = 1;
    let max = 9;

    for _ in 0..100 {
        let curr = cups[0];
        let pickup = cups.drain(1..4).collect::<Vec<_>>();

        let mut dest = if curr == min { max } else { curr - 1 };
        while pickup.contains(&dest) {
            dest = if dest == min { max } else { dest - 1 };
        }

        let dest_idx = cups.iter().position(|&cup| cup == dest).unwrap();
        #[allow(clippy::range_plus_one)]
        cups.splice(dest_idx + 1..dest_idx + 1, pickup);

        cups.rotate_left(1);
    }

    let one_idx = cups.iter().position(|&cup| cup == 1).unwrap();
    cups.iter()
        .cycle()
        .skip(one_idx + 1)
        .take(cups.len() - 1)
        .fold(0, |acc, &cup| (10 * acc) + u32::from(cup))
}

fn part2(cups: &[u8]) -> u64 {
    fn make_next(cups: &[u8], total: usize) -> Vec<u32> {
        let mut next = vec![0; total + 1];

        for w in cups.windows(2) {
            next[w[0] as usize] = u32::from(w[1]);
        }

        let n = cups.len();
        let first = u32::from(cups[0]);
        let last = u32::from(cups[n - 1]);

        #[allow(clippy::cast_possible_truncation)]
        if total > n {
            next[last as usize] = (n + 1) as u32;

            #[allow(clippy::needless_range_loop)]
            for cup in n + 1..total {
                next[cup] = (cup + 1) as u32;
            }

            next[total] = first;
        } else {
            next[last as usize] = first;
        }

        next
    }

    let min = 1;
    let max = 1_000_000;

    let mut next = make_next(cups, max as usize);
    let mut curr = u32::from(cups[0]);

    for _ in 0..10_000_000 {
        let a = next[curr as usize];
        let b = next[a as usize];
        let c = next[b as usize];

        next[curr as usize] = next[c as usize];

        let mut dest = if curr == min { max } else { curr - 1 };
        while dest == a || dest == b || dest == c {
            dest = if dest == 1 { max } else { dest - 1 };
        }

        next[c as usize] = next[dest as usize];
        next[dest as usize] = a;

        curr = next[curr as usize];
    }

    let a = next[1];
    let b = next[a as usize];
    u64::from(a) * u64::from(b)
}

fn main() -> Result<()> {
    #[allow(clippy::cast_possible_truncation)]
    let cups = fs::read_to_string("in/day23.txt")?
        .chars()
        .map(|c| c.to_digit(10).ok_or_else(|| anyhow!("invalid digit '{c}'")))
        .map(|digit| digit.map(|digit| digit as u8))
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(cups.clone());
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 76_952_348);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&cups);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 72_772_522_064);
    };

    Ok(())
}
