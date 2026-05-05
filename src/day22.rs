use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use anyhow::anyhow;

#[derive(Clone, Copy)]
enum Winner {
    Player1,
    Player2,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Player {
    cards: VecDeque<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Game {
    player1: Player,
    player2: Player,
}

impl FromStr for Player {
    type Err = Error;

    fn from_str(cards: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cards: cards.lines().map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

impl FromStr for Game {
    type Err = Error;

    #[allow(clippy::similar_names)]
    fn from_str(players: &str) -> Result<Self> {
        let (player1, player2) = players
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        let player1 = player1
            .strip_prefix("Player 1:\n")
            .ok_or_else(|| anyhow!("invalid deck for player 1"))?;
        let player2 = player2
            .strip_prefix("Player 2:\n")
            .ok_or_else(|| anyhow!("invalid deck for player 2"))?;

        Ok(Self {
            player1: player1.parse()?,
            player2: player2.parse()?,
        })
    }
}

impl Player {
    fn score(&self) -> u64 {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &card)| (i + 1) as u64 * u64::from(card))
            .sum()
    }
}

impl Game {
    fn score(&self, winner: Winner) -> u64 {
        match winner {
            Winner::Player1 => self.player1.score(),
            Winner::Player2 => self.player2.score(),
        }
    }

    fn play(&mut self) -> Winner {
        while let Some((&card1, &card2)) =
            self.player1.cards.front().zip(self.player2.cards.front())
        {
            self.player1.cards.pop_front();
            self.player2.cards.pop_front();

            match card1.cmp(&card2) {
                Ordering::Greater => self.player1.cards.extend([card1, card2]),
                Ordering::Less => self.player2.cards.extend([card2, card1]),
                Ordering::Equal => unreachable!(),
            }
        }

        if self.player1.cards.is_empty() {
            Winner::Player2
        } else {
            Winner::Player1
        }
    }

    fn play_recursive(&mut self) -> Winner {
        let mut seen = HashSet::new();

        loop {
            if !seen.insert(self.clone()) {
                return Winner::Player1;
            }

            let Some((&card1, &card2)) = self.player1.cards.front().zip(self.player2.cards.front())
            else {
                break;
            };

            self.player1.cards.pop_front();
            self.player2.cards.pop_front();

            let round = if self.player1.cards.len() >= usize::from(card1)
                && self.player2.cards.len() >= usize::from(card2)
            {
                Self {
                    player1: Player {
                        cards: self
                            .player1
                            .cards
                            .iter()
                            .take(usize::from(card1))
                            .copied()
                            .collect(),
                    },
                    player2: Player {
                        cards: self
                            .player2
                            .cards
                            .iter()
                            .take(usize::from(card2))
                            .copied()
                            .collect(),
                    },
                }
                .play_recursive()
            } else if card1 > card2 {
                Winner::Player1
            } else {
                Winner::Player2
            };

            match round {
                Winner::Player1 => self.player1.cards.extend([card1, card2]),
                Winner::Player2 => self.player2.cards.extend([card2, card1]),
            }
        }

        if self.player1.cards.is_empty() {
            Winner::Player2
        } else {
            Winner::Player1
        }
    }
}

fn part1(mut game: Game) -> u64 {
    let winner = game.play();
    game.score(winner)
}

fn part2(mut game: Game) -> u64 {
    let winner = game.play_recursive();
    game.score(winner)
}

fn main() -> Result<()> {
    let game = Game::from_str(&fs::read_to_string("in/day22.txt")?)?;

    {
        let start = Instant::now();
        let part1 = self::part1(game.clone());
        let elapsed = start.elapsed();

        println!("Part 1: {part1} ({elapsed:?})");
        assert_eq!(part1, 33_680);
    };

    {
        let start = Instant::now();
        let part2 = self::part2(game);
        let elapsed = start.elapsed();

        println!("Part 2: {part2} ({elapsed:?})");
        assert_eq!(part2, 33_683);
    };

    Ok(())
}
