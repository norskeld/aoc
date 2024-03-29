//! [Day 7: No Space Left On Device][link]
//!
//! [link]: https://adventofcode.com/2022/day/7

use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

fn collect_sizes(input: &str) -> HashMap<PathBuf, usize> {
  let mut sizes = HashMap::<PathBuf, usize>::new();
  let mut paths = Vec::new();

  for line in input.lines() {
    if line.starts_with("$ ls") || line.starts_with("dir") {
      continue;
    }

    let parts = line.split_whitespace().collect::<Vec<_>>();

    match *parts.as_slice() {
      | ["$", "cd", ".."] => {
        paths.pop();
      },
      | ["$", "cd", name] => {
        paths.push(name);
      },
      | [size, _] => {
        let size = size.parse::<usize>().unwrap();

        for index in 0..paths.len() {
          let path = PathBuf::from_iter(&paths[..=index]);
          *sizes.entry(path).or_insert(0) += size;
        }
      },
      | _ => {},
    };
  }

  sizes
}

fn solve_part_one(input: &str) -> usize {
  const SIZE_LIMIT: usize = 100_000;

  let sizes = collect_sizes(input);

  sizes.into_values().filter(|size| *size <= SIZE_LIMIT).sum()
}

fn solve_part_two(input: &str) -> usize {
  const DISK_SPACE: usize = 70_000_000;
  const UNUSED_SPACE: usize = 30_000_000;

  let sizes = collect_sizes(input);

  let root = sizes.get(Path::new("/")).unwrap();
  let available = DISK_SPACE - root;

  sizes
    .into_values()
    .filter(|size| (available + size) >= UNUSED_SPACE)
    .min()
    .unwrap_or_default()
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 7: No Space Left On Device",
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
    assert_eq!(solve_part_one(EXAMPLE), 95437);
    assert_eq!(solve_part_two(EXAMPLE), 24933642);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 2104783);
    assert_eq!(solve_part_two(INPUT), 5883165);
  }
}
