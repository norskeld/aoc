//! [Day 6: Wait For It][link]
//!
//! [link]: https://adventofcode.com/2023/day/6

use std::str::FromStr;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum ParseError {
  InvalidInput,
}

#[derive(Debug)]
struct Race {
  time: usize,
  record: usize,
}

impl Race {
  fn permutations(&self) -> usize {
    let (is_even, max) = if self.time % 2 == 0 {
      (true, (self.time / 2).pow(2))
    } else {
      (false, self.time / 2 * (self.time / 2 + 1))
    };

    let mut perms = 0;
    let mut race_diff = 0;
    let mut incrementor = if is_even { 1 } else { 0 };
    let record_diff = max - self.record;

    race_diff += incrementor;

    while race_diff < record_diff {
      incrementor += 2;
      race_diff += incrementor;
      perms += 1;
    }

    if is_even {
      perms * 2 + 1
    } else {
      perms * 2
    }
  }
}

#[derive(Debug)]
struct Document {
  races: Vec<Race>,
}

impl Document {
  fn total_permutations(&self) -> usize {
    self.races.iter().map(Race::permutations).product()
  }
}

#[derive(Debug, Default)]
struct Table {
  times: Vec<usize>,
  records: Vec<usize>,
}

impl Table {
  fn into_document(self) -> Document {
    Document {
      races: self
        .times
        .into_iter()
        .zip(self.records)
        .map(|(time, record)| Race { time, record })
        .collect::<Vec<_>>(),
    }
  }

  fn into_sr_document(self) -> Document {
    fn fuse(vec: Vec<usize>) -> usize {
      vec
        .into_iter()
        .fold(0, |acc, x| acc * 10usize.pow(x.ilog10() + 1) + x)
    }

    Document {
      races: vec![Race {
        time: fuse(self.times),
        record: fuse(self.records),
      }],
    }
  }
}

impl FromStr for Table {
  type Err = ParseError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    fn parse(line: &str, prefix: &str) -> Vec<usize> {
      line
        .strip_prefix(prefix)
        .map(str::trim)
        .map(|line| {
          line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>()
        })
        .unwrap_or_default()
    }

    input
      .split_once('\n')
      .map(|(times, distances)| {
        let times = parse(times, "Time:");
        let records = parse(distances, "Distance:");

        Table { times, records }
      })
      .ok_or(ParseError::InvalidInput)
  }
}

fn solve_part_one(input: &str) -> u32 {
  let table = Table::from_str(input).unwrap_or_default();
  let total = table.into_document().total_permutations();

  total as u32
}

fn solve_part_two(input: &str) -> u32 {
  let table = Table::from_str(input).unwrap_or_default();
  let total = table.into_sr_document().total_permutations();

  total as u32
}

pub fn solution<'s>() -> Solution<'s, u32, u32> {
  Solution {
    title: "Day 6: Wait For It",
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
    assert_eq!(solve_part_one(EXAMPLE), 288);
    assert_eq!(solve_part_two(EXAMPLE), 71503);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 227850);
    assert_eq!(solve_part_two(INPUT), 0);
  }
}
