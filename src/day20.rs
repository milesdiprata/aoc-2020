use std::collections::HashMap;
use std::fs;
use std::iter;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Debug)]
struct Grid {
    cells: Vec<Vec<bool>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge {
    bits: u16,
}

#[derive(Debug)]
struct Tile {
    id: u64,
    grid: Grid,
    edges: [Edge; Self::EDGES],
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Edge")
            .field(&format!("{:010b}", self.bits))
            .finish()
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            if y > 0 {
                writeln!(f)?;
            }

            for &cell in row {
                if cell {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        Ok(())
    }
}

impl<I: Iterator<Item = bool>> From<I> for Edge {
    fn from(cells: I) -> Self {
        Self {
            bits: cells.fold(0, |acc, cell| (acc << 1) | u16::from(cell)),
        }
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(tile: &str) -> Result<Self> {
        let id = tile
            .lines()
            .nth(0)
            .ok_or_else(|| anyhow!("missing tile ID line"))?
            .strip_prefix("Tile ")
            .and_then(|id| id.strip_suffix(':'))
            .ok_or_else(|| anyhow!("invalid tile ID"))?
            .parse()?;

        let mut grid = Grid {
            cells: vec![vec![false; 10]; 10],
        };

        for (y, line) in tile.lines().skip(1).enumerate() {
            for (x, c) in line.char_indices() {
                grid.cells[y][x] = c == '#';
            }
        }

        let edges = [grid.top(), grid.right(), grid.bottom(), grid.left()];

        Ok(Self { id, grid, edges })
    }
}

impl Edge {
    fn rev(self) -> Self {
        Self {
            bits: (0..10).fold(0, |acc, i| acc | (((self.bits >> i) & 1) << (10 - 1 - i))),
        }
    }

    fn normalize(self) -> Self {
        self.min(self.rev())
    }
}

impl Grid {
    const fn size(&self) -> usize {
        self.cells.len()
    }

    fn top(&self) -> Edge {
        Edge::from((0..self.size()).map(|x| self.cells[0][x]))
    }

    fn right(&self) -> Edge {
        Edge::from((0..self.size()).map(|y| self.cells[y][self.size() - 1]))
    }

    fn bottom(&self) -> Edge {
        Edge::from((0..self.size()).map(|x| self.cells[self.size() - 1][x]))
    }

    fn left(&self) -> Edge {
        Edge::from((0..self.size()).map(|y| self.cells[y][0]))
    }

    fn rotate_cw(&self) -> Self {
        let n = self.size();
        Self {
            cells: (0..n)
                .map(|y| (0..n).map(|x| self.cells[n - 1 - x][y]).collect())
                .collect(),
        }
    }

    fn flip_horizontal(&self) -> Self {
        Self {
            cells: self
                .cells
                .iter()
                .map(|row| row.iter().rev().copied().collect())
                .collect(),
        }
    }

    fn orientations(&self) -> impl Iterator<Item = Self> {
        [self.clone(), self.flip_horizontal()]
            .into_iter()
            .flat_map(|start| iter::successors(Some(start), |grid| Some(grid.rotate_cw())).take(4))
    }
}

impl Tile {
    const EDGES: usize = 4;
}

fn part1(tiles: &[Tile]) -> u64 {
    let mut edge_count = HashMap::new();
    for tile in tiles {
        for edge in tile.edges {
            *edge_count.entry(edge.normalize()).or_insert(0_usize) += 1;
        }
    }

    let is_border = |edge: Edge| edge_count[&edge.normalize()] == 1;
    let is_corner = |tile: &Tile| tile.edges.iter().filter(|&&edge| is_border(edge)).count() == 2;

    tiles
        .iter()
        .filter(|&tile| is_corner(tile))
        .map(|tile| tile.id)
        .product()
}

fn part2(tiles: &[Tile]) -> usize {
    const MONSTER: [&[u8]; 3] = [
        b"                  # ",
        b"#    ##    ##    ###",
        b" #  #  #  #  #  #   ",
    ];

    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    let n = (tiles.len() as f64).sqrt() as usize;

    let mut edge_to_tile_idx = HashMap::new();
    for (i, tile) in tiles.iter().enumerate() {
        for edge in tile.edges {
            edge_to_tile_idx
                .entry(edge.normalize())
                .or_insert_with(Vec::new)
                .push(i);
        }
    }

    let is_border = |edge: Edge| edge_to_tile_idx[&edge.normalize()].len() == 1;
    let oriented = |tile: &Tile, top: Option<Edge>, left: Option<Edge>| {
        tile.grid
            .orientations()
            .find(|grid| {
                top.is_none_or(|top| top == grid.top())
                    && left.is_none_or(|left| left == grid.left())
            })
            .unwrap()
    };

    let corner_idx = tiles
        .iter()
        .position(|tile| tile.edges.iter().filter(|&&edge| is_border(edge)).count() == 2)
        .unwrap();
    let corner = tiles[corner_idx]
        .grid
        .orientations()
        .find(|grid| is_border(grid.top()) && is_border(grid.left()))
        .unwrap();

    let mut placed_idxs = vec![corner_idx];
    let mut placed = vec![corner];

    for i in 1..n * n {
        let (r, c) = (i / n, i % n);
        let top_idx = (r > 0).then(|| ((r - 1) * n) + c);
        let left_idx = (c > 0).then(|| (r * n) + c - 1);
        let prev_idx = placed_idxs[left_idx.or(top_idx).unwrap()];

        let top_edge = top_idx.map(|idx| placed[idx].bottom());
        let left_edge = left_idx.map(|idx| placed[idx].right());
        let next_edge = left_edge.or(top_edge).unwrap().normalize();

        let next_idx = edge_to_tile_idx[&next_edge]
            .iter()
            .copied()
            .find(|&idx| idx != prev_idx)
            .unwrap();

        placed_idxs.push(next_idx);
        placed.push(oriented(&tiles[next_idx], top_edge, left_edge));
    }

    let inner = tiles[0].grid.size() - 2;
    let size = n * inner;
    let image = Grid {
        cells: (0..size)
            .map(|y| {
                (0..size)
                    .map(|x| {
                        placed[((y / inner) * n) + (x / inner)].cells[(y % inner) + 1] // Skips top border
                            [(x % inner) + 1] // Skips left border
                    })
                    .collect()
            })
            .collect(),
    };

    let offsets: Vec<(usize, usize)> = MONSTER
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, &b)| (b == b'#').then_some((y, x)))
        })
        .collect();
    let (mh, mw) = (MONSTER.len(), MONSTER[0].len());

    image
        .orientations()
        .find_map(|grid| {
            let s = grid.size();
            let count = (0..=s - mh)
                .flat_map(|y| (0..=s - mw).map(move |x| (y, x)))
                .filter(|&(y, x)| offsets.iter().all(|&(dy, dx)| grid.cells[y + dy][x + dx]))
                .count();

            (count > 0).then(|| {
                let total: usize = grid.cells.iter().flatten().filter(|&&b| b).count();
                total - (count * offsets.len())
            })
        })
        .unwrap()
}

fn main() -> Result<()> {
    let tiles = fs::read_to_string("in/day20.txt")?
        .split("\n\n")
        .map(Tile::from_str)
        .collect::<Result<Vec<_>>>()?;

    {
        let start = Instant::now();
        let part1 = self::part1(&tiles);
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 20_913_499_394_191);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(&tiles);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 2_209);
    };

    Ok(())
}
