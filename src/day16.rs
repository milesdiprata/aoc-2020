use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug)]
struct Rule {
    name: String,
    valid: [RangeInclusive<u64>; 2],
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u64>,
}

#[derive(Debug)]
struct Notes {
    rules: Vec<Rule>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(rule: &str) -> Result<Self> {
        fn parse_range(range: &str) -> Result<RangeInclusive<u64>> {
            let Some((start, end)) = range.split_once('-') else {
                bail!("invalid range '{range}'");
            };

            Ok(start.parse()?..=end.parse()?)
        }

        let Some((name, ranges)) = rule.split_once(": ") else {
            bail!("invalid rule '{rule}'")
        };

        let Some((r1, r2)) = ranges.split_once(" or ") else {
            bail!("invalid pair of ranges '{ranges}'");
        };

        Ok(Self {
            name: name.to_string(),
            valid: [parse_range(r1)?, parse_range(r2)?],
        })
    }
}

impl FromStr for Ticket {
    type Err = Error;

    fn from_str(fields: &str) -> Result<Self> {
        Ok(Self {
            fields: fields
                .split(',')
                .map(str::parse)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl FromStr for Notes {
    type Err = Error;

    fn from_str(notes: &str) -> Result<Self> {
        let Some((rules, rest)) = notes.split_once("\n\n") else {
            bail!("missing rules section in notes");
        };

        let Some((ticket, nearby_tickets)) = rest.split_once("\n\n") else {
            bail!("missing ticket section in notes");
        };

        Ok(Self {
            rules: rules.lines().map(str::parse).collect::<Result<_>>()?,
            ticket: ticket
                .lines()
                .nth(1)
                .ok_or_else(|| anyhow!("missing my ticket"))?
                .parse()?,
            nearby_tickets: nearby_tickets
                .lines()
                .skip(1)
                .map(str::parse)
                .collect::<Result<_>>()?,
        })
    }
}

impl Rule {
    fn is_compliant(&self, field: u64) -> bool {
        self.valid.iter().any(|range| range.contains(&field))
    }
}

impl Ticket {
    fn is_valid(&self, rule: &Rule) -> bool {
        self.fields.iter().all(|&field| rule.is_compliant(field))
    }
}

fn part1(rules: &[Rule], nearby_tickets: &[Ticket]) -> u64 {
    let mut scanning_error = 0;

    for ticket in nearby_tickets {
        for &field in &ticket.fields {
            if rules
                .iter()
                .all(|rule| rule.valid.iter().all(|range| !range.contains(&field)))
            {
                scanning_error += field;
            }
        }
    }

    scanning_error
}

fn part2(notes: Notes) -> u64 {
    let rules = notes.rules;
    let ticket = notes.ticket;
    let tickets = notes
        .nearby_tickets
        .iter()
        .filter(|&ticket| rules.iter().any(|rule| ticket.is_valid(rule)))
        .chain([&ticket])
        .collect::<Vec<_>>();
    let cols = tickets.first().unwrap().fields.len();

    let mut candidates = vec![HashSet::new(); cols];
    for (col, set) in candidates.iter_mut().enumerate() {
        for (i, rule) in rules.iter().enumerate() {
            if tickets
                .iter()
                .map(|&ticket| ticket.fields[col])
                .all(|field| rule.is_compliant(field))
            {
                set.insert(i);
            }
        }
    }

    let mut rule_assignments = vec![None; cols];
    while candidates.iter().any(|set| !set.is_empty()) {
        let (col, rule_to_assign) = candidates
            .iter()
            .enumerate()
            .find_map(|(col, set)| (set.len() == 1).then(|| (col, *set.iter().next().unwrap())))
            .expect("will not converge without backtracking");

        rule_assignments[col] = Some(rule_to_assign);

        for set in &mut candidates {
            set.remove(&rule_to_assign);
        }
    }

    let mut result = 1;
    for (col, i) in rule_assignments.into_iter().enumerate() {
        if rules[i.unwrap()].name.starts_with("departure") {
            result *= ticket.fields[col];
        }
    }

    result
}

fn main() -> Result<()> {
    let notes = Notes::from_str(&fs::read_to_string("in/day16.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(&notes.rules, &notes.nearby_tickets);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 29_019);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(notes);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 517_827_547_723);
    };

    Ok(())
}
