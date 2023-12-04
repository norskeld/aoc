//! [Day 4: Scratchcards][link]
//!
//! [link]: https://adventofcode.com/2023/day/4

use std::cmp::min;
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
  let mut result = 0;

  for card in input.lines().filter_map(parse) {
    let points = card.points();

    if points > 0 {
      result += 2u32.pow(points - 1);
    }
  }

  result
}

fn solve_part_two(input: &str) -> u32 {
  let cards: Vec<Card> = input.lines().filter_map(parse).collect();
  let mut totals = vec![1; cards.len()];

  for idx in 1..cards.len() {
    let card = &cards[idx - 1];
    let points = card.points();

    for x in idx..min(totals.len(), idx + (points as usize)) {
      totals[x] += totals[idx - 1];
    }
  }

  totals.iter().sum()
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
    assert_eq!(solve_part_two(EXAMPLE), 30);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 32001);
    assert_eq!(solve_part_two(INPUT), 5037841);
  }
}
