//! [Day 2: Rock Paper Scissors][link]
//!
//! Overengineered the hell out of it. But it works!
//!
//! [link]: https://adventofcode.com/2022/day/2

use aoc::{Part, Solution};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum ParseError {
  UnknownOutcome,
  UnknownShape,
  InvalidRound,
}

#[derive(Debug, PartialEq)]
enum Outcome {
  Loss = 0,
  Draw = 3,
  Win = 6,
}

impl TryFrom<&u8> for Outcome {
  type Error = ParseError;

  fn try_from(s: &u8) -> Result<Self, Self::Error> {
    match *s as char {
      | 'X' => Ok(Outcome::Loss),
      | 'Y' => Ok(Outcome::Draw),
      | 'Z' => Ok(Outcome::Win),
      | _ => Err(ParseError::UnknownOutcome),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Shape {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

impl Shape {
  /// Calculates the [Outcome] of crossing [Shape] (self) with another [Shape].
  fn to_outcome(self, other: &Self) -> Outcome {
    if self == *other {
      Outcome::Draw
    } else {
      match (self, other) {
        | (Shape::Rock, Shape::Paper) => Outcome::Win,
        | (Shape::Rock, Shape::Scissors) => Outcome::Loss,
        | (Shape::Paper, Shape::Rock) => Outcome::Loss,
        | (Shape::Paper, Shape::Scissors) => Outcome::Win,
        | (Shape::Scissors, Shape::Rock) => Outcome::Win,
        | (Shape::Scissors, Shape::Paper) => Outcome::Loss,
        | _ => {
          unreachable!("This case is unreachable since other cases where checked and eliminated.")
        },
      }
    }
  }

  /// Takes an [Outcome] and depending on [Shape] (self) returns a shape that will satisfy the
  /// [Outcome], e.g. if the [Outcome] is [Outcome::Loss] and [Shape] is [Shape::Rock], then the
  /// result will be [Shape::Scissors], because we aim to lose and rock beats scissors.
  fn with_outcome(&self, out: &Outcome) -> Shape {
    if *out == Outcome::Draw {
      *self
    } else {
      match (self, out) {
        | (Shape::Rock, Outcome::Loss) => Shape::Scissors,
        | (Shape::Rock, Outcome::Win) => Shape::Paper,
        | (Shape::Paper, Outcome::Loss) => Shape::Rock,
        | (Shape::Paper, Outcome::Win) => Shape::Scissors,
        | (Shape::Scissors, Outcome::Loss) => Shape::Paper,
        | (Shape::Scissors, Outcome::Win) => Shape::Rock,
        | _ => {
          unreachable!("This case is unreachable since other cases where checked and eliminated.")
        },
      }
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
      | _ => Err(ParseError::UnknownShape),
    }
  }
}

struct Round {
  shape: Shape,
  outcome: Outcome,
}

impl Round {
  fn try_parse_for(part: Part) -> impl Fn(&[u8]) -> Result<Self, ParseError> {
    move |bytes| {
      match bytes {
        | [left, .., right] => {
          let (shape, outcome) = match part {
            | Part::One => {
              let opponent = Shape::try_from(left)?;
              let shape = Shape::try_from(right)?;
              let outcome = opponent.to_outcome(&shape);

              (shape, outcome)
            },
            | Part::Two => {
              let outcome = Outcome::try_from(right)?;
              let opponent = Shape::try_from(left)?;
              let shape = opponent.with_outcome(&outcome);

              (shape, outcome)
            },
          };

          Ok(Round { shape, outcome })
        },
        | _ => Err(ParseError::InvalidRound),
      }
    }
  }
}

#[derive(Default)]
struct Output {
  result: usize,
}

impl FromIterator<Round> for Output {
  fn from_iter<I>(it: I) -> Self
  where
    I: IntoIterator<Item = Round>,
  {
    it.into_iter()
      .fold(Self::default(), |mut acc, Round { outcome, shape, .. }| {
        acc.result += outcome as usize + shape as usize;
        acc
      })
  }
}

fn solve_part_one(input: &str) -> usize {
  let output = input
    .lines()
    .map(str::as_bytes)
    .map(Round::try_parse_for(Part::One))
    .map(Result::unwrap)
    .collect::<Output>();

  output.result
}

fn solve_part_two(input: &str) -> usize {
  let output = input
    .lines()
    .map(str::as_bytes)
    .map(Round::try_parse_for(Part::Two))
    .map(Result::unwrap)
    .collect::<Output>();

  output.result
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 2: Rock Paper Scissors",
    part_one: solve_part_one(INPUT),
    part_two: solve_part_two(INPUT),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = include_str!("example.txt");

  #[test]
  fn test_examples() {
    assert_eq!(solve_part_one(EXAMPLE), 15);
    assert_eq!(solve_part_two(EXAMPLE), 12);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 11063);
    assert_eq!(solve_part_two(INPUT), 10349);
  }
}
