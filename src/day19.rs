use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
enum RuleType {
    Match(char),
    SubRules(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct Rule {
    id: usize,
    rule: RuleType,
}

impl FromStr for RuleType {
    type Err = Error;

    fn from_str(rule: &str) -> Result<Self> {
        if let Some(rule) = rule
            .strip_prefix('\"')
            .and_then(|rule| rule.strip_suffix('\"'))
        {
            Ok(Self::Match(
                rule.chars()
                    .next()
                    .ok_or_else(|| anyhow!("empty match rule'"))?,
            ))
        } else {
            let subrules = if let Some((subrules_i, subrules_j)) = rule.split_once(" | ") {
                vec![subrules_i, subrules_j]
            } else {
                vec![rule]
            };

            let subrules = subrules
                .into_iter()
                .map(|subrules| {
                    subrules
                        .split_ascii_whitespace()
                        .map(str::parse)
                        .collect::<Result<Vec<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Self::SubRules(subrules))
        }
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(rule: &str) -> Result<Self> {
        let (id, rule) = rule
            .split_once(": ")
            .ok_or_else(|| anyhow!("invalid rule '{rule}'"))?;

        Ok(Self {
            id: id.parse()?,
            rule: rule.parse()?,
        })
    }
}

fn parse() -> Result<(Vec<Rule>, Vec<String>)> {
    let input = fs::read_to_string("in/day19.txt")?;
    let (rules, msgs) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input"))?;

    let mut rules = rules
        .lines()
        .map(Rule::from_str)
        .collect::<Result<Vec<_>>>()?;
    rules.sort_by_key(|rule| rule.id);
    for (i, rule) in rules.iter().enumerate() {
        assert_eq!(i, rule.id);
    }

    let msgs = msgs.lines().map(str::to_string).collect();

    Ok((rules, msgs))
}

fn matches<'a>(rules: &[Rule], id: usize, msg: &'a str) -> Vec<&'a str> {
    match &rules[id].rule {
        &RuleType::Match(c) => {
            if msg.starts_with(c) {
                vec![&msg[c.len_utf8()..]]
            } else {
                vec![]
            }
        }
        RuleType::SubRules(subrules) => {
            let mut out = Vec::new();
            for subrule in subrules {
                let mut suffixes = vec![msg];
                for &id in subrule {
                    suffixes = suffixes
                        .into_iter()
                        .flat_map(|suffix| self::matches(rules, id, suffix))
                        .collect();
                    if suffixes.is_empty() {
                        break;
                    }
                }

                out.extend(suffixes);
            }
            out
        }
    }
}

fn matches_rule_0(rules: &[Rule], msgs: &[String]) -> usize {
    msgs.iter()
        .filter(|&msg| self::matches(rules, 0, msg).into_iter().any(str::is_empty))
        .count()
}

fn part1(rules: &[Rule], msgs: &[String]) -> usize {
    self::matches_rule_0(rules, msgs)
}

fn part2(rules: &mut [Rule], msgs: &[String]) -> usize {
    rules[8].rule = "42 | 42 8".parse().unwrap();
    rules[11].rule = "42 31 | 42 11 31".parse().unwrap();
    self::matches_rule_0(rules, msgs)
}

fn main() -> Result<()> {
    let (mut rules, msgs) = self::parse()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&rules, &msgs);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 250);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&mut rules, &msgs);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 359);
    };

    Ok(())
}
