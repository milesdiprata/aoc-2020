use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;
use strum::EnumString;

#[derive(Clone, Copy, Debug, EnumString)]
enum Op {
    #[strum(serialize = "acc")]
    Acc,
    #[strum(serialize = "jmp")]
    Jmp,
    #[strum(serialize = "nop")]
    Nop,
}

#[derive(Clone, Copy)]
struct Instr {
    op: Op,
    arg: i32,
}

#[derive(Clone, Copy)]
enum ExitCond {
    Halted,
    Loop,
}

#[derive(Clone, Copy)]
struct Exit {
    cond: ExitCond,
    acc: i32,
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(instr: &str) -> Result<Self> {
        let (op, arg) = instr
            .split_once(' ')
            .ok_or_else(|| anyhow!("invalid instruction '{instr}'"))?;

        Ok(Self {
            op: op.parse()?,
            arg: arg.parse()?,
        })
    }
}

fn run(program: &[Instr]) -> Result<Exit> {
    let mut pc = 0;
    let mut acc = 0;
    let mut seen = vec![false; program.len()];

    loop {
        if pc > program.len() {
            return Err(anyhow!("invalid program"));
        }

        if pc == program.len() {
            return Ok(Exit {
                cond: ExitCond::Halted,
                acc,
            });
        }

        if seen[pc] {
            return Ok(Exit {
                cond: ExitCond::Loop,
                acc,
            });
        }

        seen[pc] = true;

        let Instr { op, arg } = program[pc];

        match op {
            Op::Acc => acc += arg,
            Op::Jmp => {
                pc = pc
                    .checked_add_signed(arg as isize)
                    .ok_or_else(|| anyhow!("invalid program"))?;
                continue;
            }
            Op::Nop => {}
        }

        pc += 1;
    }
}

fn part1(program: &[Instr]) -> i32 {
    let Exit { acc, .. } = self::run(program).unwrap();
    acc
}

fn part2(program: &[Instr]) -> i32 {
    let mut program = program.to_vec();

    for i in 0..program.len() {
        let original = program[i].op;
        let flipped = match program[i].op {
            Op::Acc => continue,
            Op::Jmp => Op::Nop,
            Op::Nop => Op::Jmp,
        };

        program[i].op = flipped;

        if let Exit {
            cond: ExitCond::Halted,
            acc,
        } = self::run(&program).unwrap()
        {
            return acc;
        }

        program[i].op = original;
    }

    unreachable!("no flip condition makes program terminate")
}

fn main() -> Result<()> {
    let program = fs::read_to_string("in/day8.txt")?
        .lines()
        .map(Instr::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&program);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 1_475);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&program);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 1_270);
    };

    Ok(())
}
