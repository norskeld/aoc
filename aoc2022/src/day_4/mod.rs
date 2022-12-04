//! [Day 4: Camp Cleanup][link]
//!
//! Const generics are completely unnecessary here, and overall it could be reduced to something
//! like 80 loc, but...
//!
//! [link]: https://adventofcode.com/2022/day/4

use std::{ops::RangeInclusive, str::FromStr};

use crate::Solution;

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

enum ParseError {
  InvalidAssignment,
  InvalidPair,
}

struct Pair(u64, u64);

impl SplittableTo for Pair {}

impl Pair {
  fn from_tuple((start, end): (u64, u64)) -> Pair {
    Pair(start, end)
  }

  fn to_range_inc(&self) -> RangeInclusive<u64> {
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

struct Output<const M: bool> {
  result: u64,
}

impl<const M: bool> Output<M> {
  const fn new() -> Self {
    Output { result: 0 }
  }
}

impl<const M: bool> FromIterator<Assignment> for Output<M> {
  fn from_iter<I>(it: I) -> Self
  where
    I: IntoIterator<Item = Assignment>,
  {
    it.into_iter()
      .fold(Self::new(), |mut acc, Assignment(first, second)| {
        let add = match M {
          | true => first.subsumes(&second),
          | false => first.overlaps(&second),
        };

        acc.result += add as u64;
        acc
      })
  }
}

fn solve<const M: bool>(s: &str) -> u64 {
  let output = s
    .lines()
    .into_iter()
    .filter_map(|line| line.parse::<Assignment>().ok())
    .collect::<Output<M>>();

  output.result
}

/// `M` is a const generic that acts as a binary switch. It is needed to select which method to
/// invoke on [Pair]s: either `subsumes` (when `true`) or `overlaps` (when `false`).
pub fn solution<'s>() -> Solution<'s, u64, u64> {
  Solution {
    title: "Day 4: Camp Cleanup",
    part_one: solve::<true>(INPUT),
    part_two: solve::<false>(INPUT),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = include_str!("example.txt");

  #[test]
  fn test_examples() {
    assert_eq!(solve::<true>(EXAMPLE), 2);
    assert_eq!(solve::<false>(EXAMPLE), 4);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve::<true>(INPUT), 536);
    assert_eq!(solve::<false>(INPUT), 536);
  }
}
