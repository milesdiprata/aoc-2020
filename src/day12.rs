use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use strum::EnumString;

#[derive(Clone, Copy, Debug, EnumString)]
enum Action {
    #[strum(serialize = "N")]
    North,
    #[strum(serialize = "S")]
    South,
    #[strum(serialize = "E")]
    East,
    #[strum(serialize = "W")]
    West,
    #[strum(serialize = "L")]
    Left,
    #[strum(serialize = "R")]
    Right,
    #[strum(serialize = "F")]
    Forward,
}

#[derive(Clone, Copy, Debug)]
struct Instr {
    action: Action,
    val: i32,
}

#[derive(Clone, Debug)]
struct Ship {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

#[derive(Clone, Debug)]
struct ShipWithWaypoint {
    ship: Ship,
    x_waypoint: i32,
    y_waypoint: i32,
}

trait Rotate {
    fn rotate_ccw(self, degrees: i32) -> Self;
    fn rotate_cw(self, degrees: i32) -> Self;
}

impl FromStr for Instr {
    type Err = Error;

    fn from_str(instr: &str) -> Result<Self> {
        let action = instr[0..1].parse()?;
        let val = instr[1..].parse()?;
        Ok(Self { action, val })
    }
}

impl Rotate for (i32, i32) {
    fn rotate_ccw(self, degrees: i32) -> Self {
        let (mut x, mut y) = self;
        for _ in 0..degrees / 90 {
            (x, y) = (-y, x);
        }
        (x, y)
    }

    fn rotate_cw(self, degrees: i32) -> Self {
        let (mut x, mut y) = self;
        for _ in 0..degrees / 90 {
            (x, y) = (y, -x);
        }
        (x, y)
    }
}

impl Ship {
    const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
        }
    }

    fn simulate(&mut self, instrs: &[Instr]) -> i32 {
        for &instr in instrs {
            match instr.action {
                Action::North => self.y += instr.val,
                Action::South => self.y -= instr.val,
                Action::East => self.x += instr.val,
                Action::West => self.x -= instr.val,
                Action::Left => (self.dx, self.dy) = (self.dx, self.dy).rotate_ccw(instr.val),
                Action::Right => (self.dx, self.dy) = (self.dx, self.dy).rotate_cw(instr.val),
                Action::Forward => {
                    self.x += instr.val * self.dx;
                    self.y += instr.val * self.dy;
                }
            }
        }

        self.x.abs() + self.y.abs()
    }
}

impl ShipWithWaypoint {
    const fn new() -> Self {
        let ship = Ship::new();
        Self {
            x_waypoint: ship.x + 10,
            y_waypoint: ship.y + 1,
            ship,
        }
    }

    fn simulate(&mut self, instrs: &[Instr]) -> i32 {
        for &instr in instrs {
            match instr.action {
                Action::North => self.y_waypoint += instr.val,
                Action::South => self.y_waypoint -= instr.val,
                Action::East => self.x_waypoint += instr.val,
                Action::West => self.x_waypoint -= instr.val,
                Action::Left => {
                    (self.x_waypoint, self.y_waypoint) =
                        (self.x_waypoint, self.y_waypoint).rotate_ccw(instr.val);
                }
                Action::Right => {
                    (self.x_waypoint, self.y_waypoint) =
                        (self.x_waypoint, self.y_waypoint).rotate_cw(instr.val);
                }
                Action::Forward => {
                    self.ship.x += instr.val * self.x_waypoint;
                    self.ship.y += instr.val * self.y_waypoint;
                }
            }
        }

        self.ship.x.abs() + self.ship.y.abs()
    }
}

fn main() -> Result<()> {
    let instrs = fs::read_to_string("in/day12.txt")?
        .lines()
        .map(Instr::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = Ship::new().simulate(&instrs);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 904);
    };

    {
        let start = Instant::now();
        let part2 = ShipWithWaypoint::new().simulate(&instrs);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 18_747);
    };

    Ok(())
}
