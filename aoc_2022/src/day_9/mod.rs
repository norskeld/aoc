//! [Day 9: Rope Bridge][link]
//!
//! I believe implementing `FromStr` is a mistake here and parsing + populating moves could be done
//! way more efficiently, but the current version will do too.
//!
//! [link]: https://adventofcode.com/2022/day/9

use std::collections::HashSet;
use std::str::FromStr;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

type Coord = (isize, isize);

#[derive(Debug)]
enum ParseError {
  Move,
  Steps,
  Direction,
}

enum Move {
  Up(usize),
  Down(usize),
  Left(usize),
  Right(usize),
}

impl Move {
  fn to_coords(&self) -> Vec<Coord> {
    match *self {
      | Move::Up(steps) => vec![(0, 1); steps],
      | Move::Down(steps) => vec![(0, -1); steps],
      | Move::Left(steps) => vec![(-1, 0); steps],
      | Move::Right(steps) => vec![(1, 0); steps],
    }
  }
}

impl FromStr for Move {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.trim().split_once(' ') {
      | Some((direction, steps)) => {
        let steps = steps.parse::<usize>().map_err(|_| ParseError::Steps)?;

        let move_ = match direction {
          | "U" => Move::Up(steps),
          | "D" => Move::Down(steps),
          | "L" => Move::Left(steps),
          | "R" => Move::Right(steps),
          | _ => return Err(ParseError::Direction),
        };

        Ok(move_)
      },
      | None => Err(ParseError::Move),
    }
  }
}

fn process(moves: &[Move], rope_len: usize) -> usize {
  let mut rope: Vec<Coord> = vec![(0, 0); rope_len];
  let mut visited: HashSet<Coord> = HashSet::from([(0, 0)]);

  for (mx, my) in moves.iter().flat_map(Move::to_coords) {
    rope[0].0 += mx;
    rope[0].1 += my;

    for index in 1..rope_len {
      let dx = rope[index - 1].0 - rope[index].0;
      let dy = rope[index - 1].1 - rope[index].1;

      if dx.abs() < 2 && dy.abs() < 2 {
        continue;
      }

      dx.ne(&0).then(|| rope[index].0 += dx.signum());
      dy.ne(&0).then(|| rope[index].1 += dy.signum());
    }

    visited.insert(rope[rope_len - 1]);
  }

  visited.len()
}

fn solve<const N: usize>(input: &str) -> usize {
  let moves = input
    .lines()
    .map(str::parse::<Move>)
    .map(Result::unwrap)
    .collect::<Vec<Move>>();

  process(&moves, N)
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 9: Rope Bridge",
    part_one: solve::<2>(INPUT),
    part_two: solve::<10>(INPUT),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_ONE: &str = include_str!("example_one.txt");
  const EXAMPLE_TWO: &str = include_str!("example_two.txt");

  #[test]
  fn test_examples() {
    assert_eq!(solve::<2>(EXAMPLE_ONE), 13);
    assert_eq!(solve::<10>(EXAMPLE_TWO), 36);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve::<2>(INPUT), 6057);
    assert_eq!(solve::<10>(INPUT), 2514);
  }
}
