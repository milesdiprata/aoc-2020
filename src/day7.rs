use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Rule {
    color: String,
    contents: Vec<(u32, String)>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let line = line.strip_suffix('.').unwrap_or(line);
        let (color, rest) = line
            .split_once(" bags contain ")
            .ok_or_else(|| anyhow!("missing 'bags contain' in '{line}'"))?;

        let contents = if rest == "no other bags" {
            Vec::new()
        } else {
            rest.split(", ")
                .map(|part| {
                    let part = part
                        .strip_suffix(" bags")
                        .or_else(|| part.strip_suffix(" bag"))
                        .ok_or_else(|| anyhow!("missing bag suffix in '{part}'"))?;
                    let (count, inner) = part
                        .split_once(' ')
                        .ok_or_else(|| anyhow!("missing count in '{part}'"))?;
                    Ok((count.parse()?, inner.to_string()))
                })
                .collect::<Result<Vec<_>>>()?
        };

        Ok(Self {
            color: color.to_string(),
            contents,
        })
    }
}

fn part1(rules: &HashMap<String, Vec<(u32, String)>>) -> usize {
    let mut parents = HashMap::new();
    for (parent, contents) in rules {
        for (_, child) in contents {
            parents
                .entry(child.as_str())
                .or_insert_with(Vec::new)
                .push(parent.as_str());
        }
    }

    let mut queue = VecDeque::from(["shiny gold"]);
    let mut visited = HashSet::new();
    while let Some(bag) = queue.pop_front() {
        if let Some(adj) = parents.get(bag) {
            for &next in adj {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }

    visited.len()
}

fn part2(rules: &HashMap<String, Vec<(u32, String)>>) -> u64 {
    fn count(color: &str, rules: &HashMap<String, Vec<(u32, String)>>) -> u64 {
        rules[color]
            .iter()
            .map(|(n, child)| u64::from(*n) * (1 + count(child, rules)))
            .sum()
    }

    count("shiny gold", rules)
}

fn main() -> Result<()> {
    let rules = fs::read_to_string("in/day7.txt")?
        .lines()
        .map(Rule::from_str)
        .map(|rule| rule.map(|Rule { color, contents }| (color, contents)))
        .collect::<Result<HashMap<_, _>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&rules);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 233);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&rules);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 421_550);
    };

    Ok(())
}
