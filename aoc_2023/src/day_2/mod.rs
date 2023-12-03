//! [Day 2: Cube Conundrum][link]
//!
//! [link]: https://adventofcode.com/2023/day/2

use std::str::FromStr;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum ParseError {
  InvalidInput,
}

#[derive(Default)]
struct Conditions {
  reds: u32,
  greens: u32,
  blues: u32,
}

#[derive(Debug)]
enum Cube {
  Red(u32),
  Green(u32),
  Blue(u32),
}

#[derive(Debug)]
struct Set {
  cubes: Vec<Cube>,
}

impl Set {
  fn satisfies(&self, conditions: &Conditions) -> bool {
    self.cubes.iter().all(|cube| {
      match cube {
        | Cube::Red(n) => &conditions.reds >= n,
        | Cube::Green(n) => &conditions.greens >= n,
        | Cube::Blue(n) => &conditions.blues >= n,
      }
    })
  }
}

impl FromStr for Set {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut cubes: Vec<Cube> = Vec::new();

    for value in s.split(',').map(str::trim) {
      let pairs = value
        .split_ascii_whitespace()
        .map(str::trim)
        .collect::<Vec<_>>();

      let cube = match pairs.as_slice() {
        | [n, "red"] => Cube::Red(n.parse::<u32>().unwrap_or_default()),
        | [n, "green"] => Cube::Green(n.parse::<u32>().unwrap_or_default()),
        | [n, "blue"] => Cube::Blue(n.parse::<u32>().unwrap_or_default()),
        | _ => return Err(ParseError::InvalidInput),
      };

      cubes.push(cube);
    }

    Ok(Set { cubes })
  }
}

#[derive(Debug)]
struct Game {
  id: u32,
  sets: Vec<Set>,
}

impl Game {
  fn satisfies(&self, conditions: &Conditions) -> bool {
    self.sets.iter().all(|set| set.satisfies(conditions))
  }
}

fn solve_part_one(input: &str) -> u32 {
  let conditions = Conditions {
    reds: 12,
    greens: 13,
    blues: 14,
  };

  let mut result = 0;

  for line in input.lines() {
    let id = line
      .chars()
      .skip_while(|ch| !ch.is_ascii_digit()) // Skip prefix
      .take_while(|ch| ch.is_ascii_digit()) // Consume id
      .collect::<String>()
      .parse::<u32>()
      .unwrap_or_default();

    let raw = line
      .chars()
      .skip_while(|ch| ch != &':')
      .skip(1)
      .collect::<String>();

    let sets = raw
      .trim()
      .split(';')
      .filter_map(|val| val.parse::<Set>().ok())
      .collect::<Vec<_>>();

    let game = Game { id, sets };

    if game.satisfies(&conditions) {
      result += game.id;
    }
  }

  result
}

fn solve_part_two(input: &str) -> u32 {
  let mut result = 0;

  for line in input.lines() {
    let _ = line
      .chars()
      .skip_while(|ch| !ch.is_ascii_digit()) // Skip prefix
      .take_while(|ch| ch.is_ascii_digit()) // Consume id
      .collect::<String>()
      .parse::<u32>()
      .unwrap_or_default();

    let raw = line
      .chars()
      .skip_while(|ch| ch != &':')
      .skip(1)
      .collect::<String>();

    let cubes = raw
      .trim()
      .split(';')
      .filter_map(|val| val.parse::<Set>().ok())
      .flat_map(|set| set.cubes);

    let mut mreds = 0;
    let mut mgreens = 0;
    let mut mblues = 0;

    for cube in cubes {
      match cube {
        | Cube::Red(n) => mreds = mreds.max(n),
        | Cube::Green(n) => mgreens = mgreens.max(n),
        | Cube::Blue(n) => mblues = mblues.max(n),
      }
    }

    result += mreds * mgreens * mblues;
  }

  result
}

pub fn solution<'s>() -> Solution<'s, u32, u32> {
  Solution {
    title: "Day 2: Cube Conundrum",
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
    assert_eq!(solve_part_one(EXAMPLE), 8);
    assert_eq!(solve_part_two(EXAMPLE), 2286);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 2810);
    assert_eq!(solve_part_two(INPUT), 69110);
  }
}
