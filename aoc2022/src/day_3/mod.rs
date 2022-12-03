///! [Day 3: Rucksack Reorganization][link]
///
/// [link]: https://adventofcode.com/2022/day/3
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use crate::Solution;

const INPUT: &str = include_str!("input.txt");

type CharSet = HashSet<char>;
type PriorityMap = HashMap<char, u64>;

fn to_charset(s: &str) -> CharSet {
  s.chars().collect()
}

fn build_priorities(range: RangeInclusive<char>, start: usize) -> PriorityMap {
  range
    .enumerate()
    .fold(HashMap::new(), |mut acc, (index, ch)| {
      acc.insert(ch, (start + index) as u64);
      acc
    })
}

fn resolve_priority(ch: char, lc_map: &PriorityMap, uc_map: &PriorityMap) -> u64 {
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
    .map(|line| {
      // Every line seems to has even length, so split in half is okay I guess...
      let (left, right) = line.split_at(line.len() / 2);

      // Get unique chars in each compartment to find intersecting chars next.
      let left = to_charset(left);
      let right = to_charset(right);

      // Find inresecting chars (duplicates) and get their priorities from maps.
      left
        .intersection(&right)
        .map(|ch| resolve_priority(*ch, &lc_priorities, &uc_priorities))
        .sum::<u64>()
    })
    .fold(0, |mut acc, priority| {
      acc += priority;
      acc
    })
}

fn solve_part_two(s: &str) -> u64 {
  let lc_priorities = build_priorities('a'..='z', 1);
  let uc_priorities = build_priorities('A'..='Z', 27);

  let lines = s.lines().collect::<Vec<_>>();

  lines
    // Split a vector of lines into chunks of 3 elements (elves).
    .chunks(3)
    // I guess it's better to panic instead, but w/e.
    .filter_map(|groups| {
      match *groups {
        // Man I freaking love slice patterns.
        | [first, second, third] => {
          vec![first, second, third]
            .into_iter()
            .map(to_charset)
            // I'm not happy with `copied`, but w/e, it's `char`s being copied, so not a big deal.
            .reduce(|acc, set| acc.intersection(&set).copied().collect::<CharSet>())
            // This piece is cringeworthy as well.
            .map(|set| {
              set
                .into_iter()
                .map(|ch| resolve_priority(ch, &lc_priorities, &uc_priorities))
                .sum::<u64>()
            })
        },
        | _ => None,
      }
    })
    .fold(0, |mut acc, priority| {
      acc += priority;
      acc
    })
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
    assert_eq!(solve_part_two(EXAMPLE), 70);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 7446);
    assert_eq!(solve_part_two(INPUT), 2646)
  }
}
