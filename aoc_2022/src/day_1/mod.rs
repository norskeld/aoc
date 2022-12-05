//! [Day 1: Calorie Counting][link]
//!
//! [link]: https://adventofcode.com/2022/day/1

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

struct Calories<const N: usize> {
  values: [u64; N],
}

impl<const N: usize> Calories<N> {
  const fn new() -> Self {
    Self { values: [0; N] }
  }

  fn update(&mut self, next: u64) {
    self.values.sort_unstable();

    for current in self.values.iter_mut() {
      if *current < next {
        *current = next;
        return;
      }
    }
  }

  fn sum(&self) -> u64 {
    self.values.iter().sum()
  }
}

impl<const N: usize> FromIterator<u64> for Calories<N> {
  fn from_iter<I>(it: I) -> Self
  where
    I: IntoIterator<Item = u64>,
  {
    it.into_iter().fold(Self::new(), |mut acc, value| {
      acc.update(value);
      acc
    })
  }
}

fn solve<const N: usize>(input: &str) -> u64 {
  // Looks dirty, but works.
  input
    .split("\n\n")
    .map(|elf| {
      elf
        .lines()
        .map(|line| line.parse::<u64>().unwrap_or_default())
        .sum()
    })
    // Just playing around with const generics and iterators...
    .collect::<Calories<N>>()
    .sum()
}

pub fn solution<'s>() -> Solution<'s, u64, u64> {
  Solution {
    title: "Day 1: Calorie Counting",
    part_one: solve::<1>(INPUT),
    part_two: solve::<3>(INPUT),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = include_str!("example.txt");

  #[test]
  fn test_examples() {
    assert_eq!(solve::<1>(EXAMPLE), 24000);
    assert_eq!(solve::<3>(EXAMPLE), 45000);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve::<1>(INPUT), 70374);
    assert_eq!(solve::<3>(INPUT), 204610);
  }
}
