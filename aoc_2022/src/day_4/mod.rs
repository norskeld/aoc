//! [Day 4: Camp Cleanup][link]
//!
//! [link]: https://adventofcode.com/2022/day/4

use std::ops::RangeInclusive;
use std::str::FromStr;

use aoc::{Part, Solution};

const INPUT: &str = include_str!("input.txt");

trait SplittableTo {
  /// Splits a given `&str` by `delimiter` into a pair of `T`.
  fn split_to_pair<T>(s: &str, delimiter: &str) -> Result<(T, T), ParseError>
  where
    T: FromStr,
  {
    s.split_once(delimiter)
      .and_then(|(first, second)| {
        let first = first.parse::<T>().ok();
        let second = second.parse::<T>().ok();

        first.zip(second)
      })
      .ok_or(ParseError::InvalidPair)
  }
}

#[derive(Debug)]
enum ParseError {
  InvalidAssignment,
  InvalidPair,
}

struct Pair(usize, usize);

impl SplittableTo for Pair {}

impl Pair {
  fn from_tuple((start, end): (usize, usize)) -> Pair {
    Pair(start, end)
  }

  fn to_range_inc(&self) -> RangeInclusive<usize> {
    self.0..=self.1
  }

  /// Checks if one range contains another range. Inclusive.
  fn subsumes(&self, other: &Pair) -> bool {
    let this = self.to_range_inc();
    let that = other.to_range_inc();

    let l_subsumes_r = this.contains(that.start()) && this.contains(that.end());
    let r_subsumes_l = that.contains(this.start()) && that.contains(this.end());

    l_subsumes_r || r_subsumes_l
  }

  /// Checks if one range overlaps (partially or as a whole) with anoterh range.
  fn overlaps(&self, other: &Pair) -> bool {
    let this = self.to_range_inc();
    let that = other.to_range_inc();

    let l_overlaps_r = this.contains(that.start()) || this.contains(that.end());
    let r_overlaps_l = that.contains(this.start()) || that.contains(this.end());

    l_overlaps_r || r_overlaps_l
  }
}

impl FromStr for Pair {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Pair::split_to_pair(s, "-")
      .map(Pair::from_tuple)
      .map_err(|_| ParseError::InvalidPair)
  }
}

struct Assignment(Pair, Pair);

impl SplittableTo for Assignment {}

impl Assignment {
  fn from_tuple((first, second): (Pair, Pair)) -> Self {
    Assignment(first, second)
  }
}

impl FromStr for Assignment {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Assignment::split_to_pair(s, ",")
      .map(Assignment::from_tuple)
      .map_err(|_| ParseError::InvalidAssignment)
  }
}

#[derive(Default)]
struct Output {
  result: usize,
}

impl FromIterator<(Part, Assignment)> for Output {
  fn from_iter<I>(it: I) -> Self
  where
    I: IntoIterator<Item = (Part, Assignment)>,
  {
    it.into_iter().fold(
      Self::default(),
      |mut acc, (part, Assignment(first, second))| {
        let add = match part {
          | Part::One => first.subsumes(&second),
          | Part::Two => first.overlaps(&second),
        };

        acc.result += add as usize;
        acc
      },
    )
  }
}

fn solve_part_one(s: &str) -> usize {
  let output = s
    .lines()
    .map(str::parse::<Assignment>)
    .map(Result::unwrap)
    .map(|assignment| (Part::One, assignment))
    .collect::<Output>();

  output.result
}

fn solve_part_two(s: &str) -> usize {
  let output = s
    .lines()
    .map(str::parse::<Assignment>)
    .map(Result::unwrap)
    .map(|assignment| (Part::Two, assignment))
    .collect::<Output>();

  output.result
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 4: Camp Cleanup",
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
    assert_eq!(solve_part_one(EXAMPLE), 2);
    assert_eq!(solve_part_two(EXAMPLE), 4);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 536);
    assert_eq!(solve_part_two(INPUT), 845);
  }
}
