//! [Day 1: Trebuchet?!][link]
//!
//! [link]: https://adventofcode.com/2023/day/1

use std::collections::HashMap;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

fn solve_part_one(input: &str) -> u32 {
  let mut result = 0;

  for line in input.lines() {
    let first = line.chars().find_map(|ch| ch.to_digit(10)).unwrap();
    let last = line.chars().rev().find_map(|ch| ch.to_digit(10)).unwrap();

    result += first * 10 + last;
  }

  result
}

fn solve_part_two(input: &str) -> u32 {
  let mappings = HashMap::from([
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("0", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("zero", 0),
  ]);

  let mut result = 0;

  for line in input.lines() {
    let mut forwards = line;
    let mut backwards = line;

    let first = 'outer: loop {
      for (prefix, num) in mappings.iter() {
        if forwards.starts_with(prefix) {
          break 'outer num;
        }
      }

      forwards = &forwards[1..];
    };

    let last = 'outer: loop {
      for (suffix, num) in mappings.iter() {
        if backwards.ends_with(suffix) {
          break 'outer num;
        }
      }

      backwards = &backwards[..backwards.len() - 1];
    };

    result += first * 10 + last;
  }

  result
}

pub fn solution<'s>() -> Solution<'s, u32, u32> {
  Solution {
    title: "Day 1: Trebuchet?!",
    part_one: solve_part_one(INPUT),
    part_two: solve_part_two(INPUT),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_1: &str = include_str!("example-1.txt");
  const EXAMPLE_2: &str = include_str!("example-2.txt");

  #[test]
  fn test_examples() {
    assert_eq!(solve_part_one(EXAMPLE_1), 142);
    assert_eq!(solve_part_two(EXAMPLE_2), 281);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 54968);
    assert_eq!(solve_part_two(INPUT), 54094);
  }
}
