///! [Day 3: Rucksack Reorganization][link]
///
/// [link]: https://adventofcode.com/2022/day/3
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use crate::Solution;

const INPUT: &str = include_str!("input.txt");

fn build_priorities(range: RangeInclusive<char>, start: usize) -> HashMap<char, u64> {
  range
    .enumerate()
    .fold(HashMap::new(), |mut acc, (index, ch)| {
      acc.insert(ch, (start + index) as u64);
      acc
    })
}

fn resolve_priority(ch: char, lc_map: &HashMap<char, u64>, uc_map: &HashMap<char, u64>) -> u64 {
  if ch.is_ascii_lowercase() {
    lc_map[&ch]
  } else {
    uc_map[&ch]
  }
}

fn solve_part_one(s: &str) -> u64 {
  let lc_priorities = build_priorities('a'..='z', 1);
  let uc_priorities = build_priorities('A'..='Z', 27);

  s.lines()
    .into_iter()
    .map(|line| {
      // Every line seems to has even length, so split in half is okay I guess...
      let (left, right) = line.split_at(line.len() / 2);

      // Get unique chars in each compartment to find intersecting chars next.
      let left = left.chars().collect::<HashSet<char>>();
      let right = right.chars().collect::<HashSet<char>>();

      // Find inresecting chars (duplicates) and get their priorities from maps.
      let priority: u64 = left
        .intersection(&right)
        .map(|ch| resolve_priority(*ch, &lc_priorities, &uc_priorities))
        .sum();

      priority
    })
    .fold(0, |mut acc, priority| {
      acc += priority;
      acc
    })
}

fn solve_part_two(_s: &str) -> u64 {
  0
}

pub fn solution<'s>() -> Solution<'s, u64, u64> {
  Solution {
    title: "Day 3: Rucksack Reorganization",
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
    assert_eq!(solve_part_one(EXAMPLE), 157);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 7446);
  }
}
