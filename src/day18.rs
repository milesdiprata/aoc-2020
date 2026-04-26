use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::bail;

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Num(u64),
    Op(Op),
    LParen,
    RParen,
}

#[derive(Clone, Copy, Debug)]
enum Frame {
    Op(Op),
    LParen,
}

#[derive(Debug)]
struct Expr {
    tokens: Vec<Token>,
}

impl TryFrom<char> for Token {
    type Error = Error;

    fn try_from(token: char) -> Result<Self> {
        Ok(match token {
            '0'..='9' => Self::Num(u64::from(token as u8 - b'0')),
            '+' => Self::Op(Op::Add),
            '*' => Self::Op(Op::Mul),
            '(' => Self::LParen,
            ')' => Self::RParen,
            _ => bail!("invalid token '{token}'"),
        })
    }
}

impl FromStr for Expr {
    type Err = Error;

    fn from_str(expr: &str) -> Result<Self> {
        Ok(Self {
            tokens: expr
                .chars()
                .filter(|token| !token.is_whitespace())
                .map(Token::try_from)
                .collect::<Result<_>>()?,
        })
    }
}

impl Op {
    const fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

impl Expr {
    fn evaluate(&self, prec: impl Fn(Op) -> u8) -> u64 {
        fn fold(nums: &mut Vec<u64>, ops: &mut Vec<Frame>) {
            let b = nums.pop().unwrap();
            let a = nums.pop().unwrap();
            let Frame::Op(op) = ops.pop().unwrap() else {
                unreachable!()
            };
            nums.push(op.apply(a, b));
        }

        let drain_above = |nums: &mut Vec<u64>, ops: &mut Vec<Frame>, min: u8| {
            while let Some(Frame::Op(top)) = ops.last() {
                if prec(*top) >= min {
                    fold(nums, ops);
                } else {
                    break;
                }
            }
        };

        let mut nums = Vec::new();
        let mut ops: Vec<Frame> = Vec::new();

        for &token in &self.tokens {
            match token {
                Token::Num(n) => nums.push(n),
                Token::LParen => ops.push(Frame::LParen),
                Token::RParen => {
                    drain_above(&mut nums, &mut ops, 0);
                    ops.pop();
                }
                Token::Op(op) => {
                    drain_above(&mut nums, &mut ops, prec(op));
                    ops.push(Frame::Op(op));
                }
            }
        }
        drain_above(&mut nums, &mut ops, 0);
        nums.pop().unwrap()
    }
}

fn part1(exprs: &[Expr]) -> u64 {
    exprs.iter().map(|e| e.evaluate(|_| 1)).sum()
}

fn part2(exprs: &[Expr]) -> u64 {
    exprs
        .iter()
        .map(|e| {
            e.evaluate(|op| match op {
                Op::Add => 2,
                Op::Mul => 1,
            })
        })
        .sum()
}

fn main() -> Result<()> {
    let exprs = fs::read_to_string("in/day18.txt")?
        .lines()
        .map(Expr::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&exprs);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 11_076_907_812_171);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&exprs);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 283_729_053_022_731);
    };

    Ok(())
}
