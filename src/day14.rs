use std::collections::HashMap;
use std::fs;
use std::iter;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

#[derive(Clone, Default)]
struct Mask {
    ones: u64,
    floating: u64,
}

#[derive(Clone, Debug)]
enum Instr {
    Mask(Mask),
    Write { addr: u64, val: u64 },
}

#[derive(Clone, Debug)]
struct Program {
    instrs: Vec<Instr>,
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mask")
            .field("ones", &format!("{:036b}", self.ones))
            .field("floating", &format!("{:036b}", self.floating))
            .finish()
    }
}

impl FromStr for Mask {
    type Err = Error;

    fn from_str(mask: &str) -> Result<Self> {
        let mut ones = 0;
        let mut dont_care = 0;

        for (i, bit) in mask.chars().rev().enumerate() {
            match bit {
                '0' => {}
                '1' => ones |= 1 << i,
                'X' => dont_care |= 1 << i,
                _ => bail!("invalid bit in mask '{bit}'"),
            }
        }

        Ok(Self {
            ones,
            floating: dont_care,
        })
    }
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(instr: &str) -> Result<Self> {
        if let Some(mask) = instr.strip_prefix("mask = ") {
            Ok(Self::Mask(mask.parse()?))
        } else if instr.starts_with("mem") {
            let (addr, val) = instr
                .split_once(" = ")
                .ok_or_else(|| anyhow!("invalid write instruction '{instr}'"))?;

            let addr = addr
                .strip_prefix("mem[")
                .and_then(|addr| addr.strip_suffix(']'))
                .ok_or_else(|| anyhow!("invalid memory address '{addr}'"))?;

            Ok(Self::Write {
                addr: addr.parse()?,
                val: val.parse()?,
            })
        } else {
            bail!("invalid instruction '{instr}'");
        }
    }
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(program: &str) -> Result<Self> {
        Ok(Self {
            instrs: program.lines().map(str::parse).collect::<Result<_>>()?,
        })
    }
}

impl Mask {
    const fn apply_val(&self, val: u64) -> u64 {
        let keep = self.ones | self.floating;
        (val & keep) | self.ones
    }

    fn apply_addr(&self, addr: u64) -> impl Iterator<Item = u64> {
        let base = (addr | self.ones) & !self.floating;

        let mut sub = self.floating;
        let mut done = false;

        iter::from_fn(move || {
            if done {
                return None;
            }

            let new = base | sub;

            if sub == 0 {
                done = true;
            } else {
                sub = (sub - 1) & self.floating;
            }

            Some(new)
        })
    }
}

impl Program {
    fn run_v1(self) -> HashMap<u64, u64> {
        let mut mask = Mask::default();
        let mut mem = HashMap::new();

        for instr in self.instrs {
            match instr {
                Instr::Mask(new) => {
                    mask = new;
                }
                Instr::Write { addr, val } => {
                    let _ = mem.insert(addr, mask.apply_val(val));
                }
            }
        }

        mem
    }

    fn run_v2(self) -> HashMap<u64, u64> {
        let mut mask = Mask::default();
        let mut mem = HashMap::new();

        for instr in self.instrs {
            match instr {
                Instr::Mask(new) => {
                    mask = new;
                }
                Instr::Write { addr, val } => {
                    for addr in mask.apply_addr(addr) {
                        mem.insert(addr, val);
                    }
                }
            }
        }

        mem
    }
}

fn part1(program: Program) -> u64 {
    program.run_v1().values().sum()
}

fn part2(program: Program) -> u64 {
    program.run_v2().values().sum()
}

fn main() -> Result<()> {
    let program = Program::from_str(&fs::read_to_string("in/day14.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(program.clone());
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 7_440_382_076_205);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(program);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 4_200_656_704_538);
    };

    Ok(())
}
