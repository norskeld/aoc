//! [Day 4: Scratchcards][link]
//!
//! [link]: https://adventofcode.com/2023/day/4

use std::collections::HashSet;

use aoc::Solution;
use chumsky::{prelude::*, text::whitespace};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Card {
  winning: HashSet<u32>,
  numbers: HashSet<u32>,
}

impl Card {
  fn points(&self) -> u32 {
    self.winning.intersection(&self.numbers).count() as u32
  }
}

fn parse(input: &str) -> Option<Card> {
  let prefix = any::<_, Simple<char>>()
    .then(take_until(just(':')))
    .padded();

  let numbers = text::digits::<_, Simple<char>>(10)
    .map(|digits| digits.parse::<u32>().unwrap_or(0))
    .separated_by(whitespace())
    .padded();

  let parser = prefix
    .ignore_then(numbers)
    .then(just('|').padded().ignored())
    .then(numbers)
    .map(|((winning, _), numbers)| {
      Card {
        winning: HashSet::from_iter(winning),
        numbers: HashSet::from_iter(numbers),
      }
    });

  parser.parse(input).ok()
}

fn solve_part_one(input: &str) -> u32 {
  input
    .lines()
    .filter_map(parse)
    .map(|card| card.points())
    .filter(|&n| n > 0)
    .map(|n| 2u32.pow(n - 1))
    .sum()
}

fn solve_part_two(_input: &str) -> u32 {
  0
}

pub fn solution<'s>() -> Solution<'s, u32, u32> {
  Solution {
    title: "Day 4: Scratchcards",
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
    assert_eq!(solve_part_one(EXAMPLE), 13);
    // assert_eq!(solve_part_two(EXAMPLE), 30);
  }

  #[test]
  fn test_input() {
    // assert_eq!(solve_part_one(INPUT), 0);
    // assert_eq!(solve_part_two(INPUT), 0);
  }
}
