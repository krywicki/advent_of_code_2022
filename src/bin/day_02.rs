use std::io::BufRead;

use aoc;

const ROCK: usize = 1;
const PAPER: usize = 2;
const SCISSORS: usize = 3;
const LOSE: usize = 0;
const DRAW: usize = 3;
const WIN: usize = 6;
const SHOULD_LOSE: usize = 1;
const SHOULD_DRAW: usize = 2;
const SHOULD_WIN: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Round {
    pub move_1: usize,
    pub move_2: usize,
}

fn main() -> aoc::Result<()> {
    let total_score: usize = aoc::open_puzzle_input("day_02.txt")?
        .lines()
        .map(Result::unwrap)
        .map(Round::from)
        .map(Round::score)
        .sum();

    println!("day 2a: {}", total_score);

    let total_score: usize = aoc::open_puzzle_input("day_02.txt")?
        .lines()
        .map(Result::unwrap)
        .map(Round::from)
        .map(Round::score_2)
        .sum();

    println!("day 2b: {}", total_score);

    Ok(())
}

impl From<String> for Round {
    fn from(value: String) -> Self {
        let moves: Vec<usize> = value
            .split_whitespace()
            .map(|s| match s.to_lowercase().as_str() {
                "a" | "x" => ROCK,
                "b" | "y" => PAPER,
                "c" | "z" => SCISSORS,
                _ => panic!("unknown rps char: {}", s),
            })
            .collect();

        Round {
            move_1: moves[0],
            move_2: moves[1],
        }
    }
}

impl Round {
    pub fn score(self) -> usize {
        if self.move_1 == self.move_2 {
            DRAW + self.move_2
        } else if (self.move_1 + 1 == self.move_2)
            || (self.move_1 == SCISSORS && self.move_2 == ROCK)
        {
            WIN + self.move_2
        } else {
            LOSE + self.move_2
        }
    }

    pub fn score_2(self) -> usize {
        match self.move_2 {
            SHOULD_DRAW => self.move_1 + DRAW,
            SHOULD_LOSE => {
                return match self.move_1 {
                    ROCK => SCISSORS,
                    _ => self.move_1 - 1,
                } + LOSE
            }
            SHOULD_WIN => {
                return match self.move_1 {
                    SCISSORS => ROCK,
                    _ => self.move_1 + 1,
                } + WIN
            }
            _ => panic!("unexpected outcome: {}", self.move_2),
        }
    }
}
