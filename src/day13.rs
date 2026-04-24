use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Notes {
    t_earliest: usize,
    bus_ids: Vec<usize>,
    no_constraints: HashSet<usize>,
}

impl FromStr for Notes {
    type Err = Error;

    fn from_str(notes: &str) -> Result<Self> {
        let (t_earliest, bus_ids) = notes
            .split_once('\n')
            .ok_or_else(|| anyhow!("invalid notes '{notes}'"))?;

        let bus_ids = bus_ids
            .split(',')
            .map(|bus| bus.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let no_constraints = bus_ids
            .iter()
            .enumerate()
            .filter_map(|(i, &parsed)| parsed.is_none().then_some(i))
            .collect();

        Ok(Self {
            t_earliest: t_earliest.parse()?,
            bus_ids: bus_ids.into_iter().flatten().collect(),
            no_constraints,
        })
    }
}

fn part1(notes: &Notes) -> usize {
    notes
        .bus_ids
        .iter()
        .filter_map(|&bus| {
            bus.checked_sub(notes.t_earliest % bus)
                .map(|wait| (wait, bus))
        })
        .min()
        .map(|(wait, bus)| wait * bus)
        .unwrap()
}

fn part2(notes: &Notes) -> usize {
    let total = notes.bus_ids.len() + notes.no_constraints.len();
    let constraints = (0..total)
        .filter(|i| !notes.no_constraints.contains(i))
        .zip(notes.bus_ids.iter())
        .map(|(i, &bus)| (i, bus));

    let mut t = 0;
    let mut step = 1;

    for (offset, bus) in constraints {
        while !(t + offset).is_multiple_of(bus) {
            t += step;
        }
        step *= bus;
    }

    t
}

fn main() -> Result<()> {
    let notes = Notes::from_str(&fs::read_to_string("in/day13.txt")?)?;
    dbg!(&notes);

    {
        let start = Instant::now();
        let part1 = self::part1(&notes);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        // assert_eq!(part1, 161);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&notes);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 213_890_632_230_818);
    };

    Ok(())
}
