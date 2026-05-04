use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = Error;

    fn from_str(food: &str) -> Result<Self> {
        let (ingredients, allergens) = food
            .split_once("(contains ")
            .ok_or_else(|| anyhow!("invalid food '{food}'"))?;

        let ingredients = ingredients
            .split_ascii_whitespace()
            .map(str::to_string)
            .collect();
        let allergens = allergens
            .strip_suffix(')')
            .ok_or_else(|| anyhow!("malformed allergen list in '{food}'"))?
            .split(", ")
            .map(str::to_string)
            .collect();

        Ok(Self {
            ingredients,
            allergens,
        })
    }
}

fn allergen_candidates(food: &[Food]) -> HashMap<&str, HashSet<&str>> {
    let mut allergen_candidates = HashMap::new();

    for f in food {
        for a in &f.allergens {
            let ingredients = f
                .ingredients
                .iter()
                .map(String::as_str)
                .collect::<HashSet<_>>();
            allergen_candidates
                .entry(a.as_str())
                .and_modify(|set: &mut HashSet<_>| set.retain(|&i| ingredients.contains(i)))
                .or_insert(ingredients);
        }
    }

    allergen_candidates
}

fn part1(food: &[Food]) -> usize {
    let allergenic_ingredients = self::allergen_candidates(food)
        .values()
        .flatten()
        .copied()
        .collect::<HashSet<_>>();

    let mut count = 0;
    for f in food {
        for i in &f.ingredients {
            if !allergenic_ingredients.contains(i.as_str()) {
                count += 1;
            }
        }
    }

    count
}

fn part2(food: &[Food]) -> String {
    let mut candidates = self::allergen_candidates(food);
    let mut assigned = Vec::with_capacity(candidates.len());

    while !candidates.is_empty() {
        let (a, &i) = candidates
            .iter()
            .find(|&(_, s)| s.len() == 1)
            .map(|(&a, s)| (a, s.iter().next().unwrap()))
            .expect("not solvable via elimination");

        assigned.push((a, i));

        candidates.remove(a);
        for s in candidates.values_mut() {
            s.remove(i);
        }
    }

    assigned.sort_unstable();
    assigned
        .into_iter()
        .map(|(_, i)| i)
        .collect::<Vec<_>>()
        .join(",")
}

fn main() -> Result<()> {
    let food = fs::read_to_string("in/day21.txt")?
        .lines()
        .map(Food::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&food);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 2_423);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&food);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, "jzzjz,bxkrd,pllzxb,gjddl,xfqnss,dzkb,vspv,dxvsp");
    };

    Ok(())
}
