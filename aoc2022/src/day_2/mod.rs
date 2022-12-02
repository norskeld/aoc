//! [Day 2: Rock Paper Scissors][link]
//!
//! Part 1 rules:
//!
//! Overengineering the hell out of it.
//!
//! ```ignore
//! A = X - Rock     - 1 point
//! B = Y - Paper    - 2 points
//! C = Z - Scissors - 3 points
//!
//! Lost - 0 points
//! Draw - 3 points
//! Win  - 6 points
//! ```
//!
//! [link]: https://adventofcode.com/2022/day/2

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum ParseError {
  ShapeError,
  RoundError,
}

enum Outcome {
  Loss,
  Draw,
  Win,
}

impl Outcome {
  pub fn get_score(&self) -> u64 {
    match self {
      | Outcome::Loss => 0,
      | Outcome::Draw => 3,
      | Outcome::Win => 6,
    }
  }
}

#[derive(Copy, Clone, PartialEq)]
enum Shape {
  Rock,
  Paper,
  Scissors,
}

impl Shape {
  fn get_score(&self) -> u64 {
    match self {
      | Shape::Rock => 1,
      | Shape::Paper => 2,
      | Shape::Scissors => 3,
    }
  }
}

impl TryFrom<&u8> for Shape {
  type Error = ParseError;

  fn try_from(s: &u8) -> Result<Self, Self::Error> {
    match *s as char {
      | 'A' | 'X' => Ok(Self::Rock),
      | 'B' | 'Y' => Ok(Self::Paper),
      | 'C' | 'Z' => Ok(Self::Scissors),
      | _ => Err(ParseError::ShapeError),
    }
  }
}

struct Round {
  pub outcome: Outcome,
  pub shape: Shape,
}

impl TryFrom<&[u8]> for Round {
  type Error = ParseError;

  fn try_from(s: &[u8]) -> Result<Self, Self::Error> {
    match s {
      | [left, .., right] => {
        let left = Shape::try_from(left)?;
        let right = Shape::try_from(right)?;

        if left == right {
          Ok(Round {
            outcome: Outcome::Draw,
            shape: right,
          })
        } else {
          let outcome = match (left, right) {
            | (Shape::Rock, Shape::Paper) => Outcome::Win,
            | (Shape::Rock, Shape::Scissors) => Outcome::Loss,
            | (Shape::Paper, Shape::Rock) => Outcome::Loss,
            | (Shape::Paper, Shape::Scissors) => Outcome::Win,
            | (Shape::Scissors, Shape::Rock) => Outcome::Win,
            | (Shape::Scissors, Shape::Paper) => Outcome::Loss,
            | _ => unreachable!("This branch is unreachable since other cases where checked and eliminated right before this.")
          };

          Ok(Round {
            outcome,
            shape: right,
          })
        }
      },
      | _ => Err(ParseError::RoundError),
    }
  }
}

struct Output {
  result: u64,
}

impl Output {
  fn new() -> Self {
    Self { result: 0 }
  }
}

impl FromIterator<Round> for Output {
  fn from_iter<I>(it: I) -> Self
  where
    I: IntoIterator<Item = Round>,
  {
    it.into_iter()
      .fold(Self::new(), |mut acc, Round { outcome, shape }| {
        acc.result += outcome.get_score() + shape.get_score();
        acc
      })
  }
}

fn solve_part_one(input: &str) -> u64 {
  let output = input
    .lines()
    .filter_map(|line| Round::try_from(line.as_bytes()).ok())
    .collect::<Output>();

  output.result
}

pub fn solution() {
  let part_one = solve_part_one(INPUT);

  println!("part one: {part_one}");
  println!("part two: --");
}
