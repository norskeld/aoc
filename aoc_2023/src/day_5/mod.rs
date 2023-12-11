//! [Day 5: If You Give A Seed A Fertilizer][link]
//!
//! [link]: https://adventofcode.com/2023/day/5

use std::str::FromStr;

use aoc::Solution;
use chumsky::prelude::*;
use chumsky::text::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum ParseError {
  InvalidInput,
}

struct SeedResult {
  tx: Option<Seed>,
  lo: Option<Seed>,
  hi: Option<Seed>,
}

impl SeedResult {
  fn new() -> Self {
    Self {
      tx: None,
      lo: None,
      hi: None,
    }
  }
}

#[derive(Debug)]
struct Seed {
  lo: isize,
  hi: isize,
}

impl Seed {
  fn split(&self, index: isize) -> (Seed, Seed) {
    (
      Seed {
        lo: self.lo,
        hi: index,
      },
      Seed {
        lo: index,
        hi: self.hi,
      },
    )
  }
}

#[derive(Debug)]
struct Mapping {
  lo: isize,
  hi: isize,
  tx: isize,
}

impl Mapping {
  fn tx(&self, x: isize) -> Option<isize> {
    if x >= self.lo && x < self.hi {
      Some(x + self.tx)
    } else {
      None
    }
  }

  fn txs(&self, seed: Seed) -> SeedResult {
    let mut seed = seed;
    let mut out = SeedResult::new();

    if seed.hi <= self.lo {
      out.lo = Some(seed);
      return out;
    }

    if seed.lo >= self.hi {
      out.hi = Some(seed);
      return out;
    }

    if seed.lo < self.lo && self.lo < seed.hi {
      let (before, after) = seed.split(self.lo);
      out.lo = Some(before);
      seed = after;
    }

    if seed.hi > self.hi && self.hi > seed.lo {
      let (before, after) = seed.split(self.hi);
      out.hi = Some(after);
      seed = before;
    }

    seed.lo += self.tx;
    seed.hi += self.tx;
    out.tx = Some(seed);

    out
  }
}

#[derive(Debug)]
struct Map {
  mappings: Vec<Mapping>,
}

impl Map {
  fn tx(&self, x: isize) -> isize {
    self
      .mappings
      .iter()
      .map(|mapping| mapping.tx(x))
      .try_fold(x, |acc, y| y.or(Some(acc)))
      .unwrap_or(0)
  }

  fn txs(&self, seeds: Vec<Seed>) -> Vec<Seed> {
    let mut seeds = seeds;
    let mut mapped = Vec::new();

    for map in self.mappings.iter() {
      let mut outside = Vec::new();

      for seed in seeds.into_iter() {
        let res = map.txs(seed);

        if let Some(s) = res.lo {
          outside.push(s)
        };

        if let Some(s) = res.tx {
          mapped.push(s)
        };

        if let Some(s) = res.hi {
          outside.push(s)
        };
      }

      seeds = outside;
    }

    mapped.append(&mut seeds);
    mapped
  }
}

#[derive(Debug, Default)]
struct Almanac {
  seeds: Vec<isize>,
  maps: Vec<Map>,
}

impl FromStr for Almanac {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let prefix = just::<_, _, Simple<char>>(':')
      .not()
      .repeated()
      .then_ignore(just(':'));

    let number = text::int::<_, Simple<char>>(10);

    let seeds = prefix
      .padded()
      .ignore_then(number.separated_by(whitespace()))
      .map(|vals| {
        vals
          .into_iter()
          .filter_map(|val| val.parse::<isize>().ok())
          .collect::<Vec<_>>()
      });

    let range = number.separated_by(whitespace()).exactly(3).map(|vals| {
      vals
        .into_iter()
        .filter_map(|val| val.parse::<isize>().ok())
        .collect::<Vec<_>>()
    });

    let map = prefix
      .ignore_then(newline())
      .ignore_then(range.separated_by(newline()))
      .map(|ranges| {
        let mappings = ranges
          .into_iter()
          .filter_map(|range| {
            match range.as_slice() {
              | &[dest, source, len] => {
                Some(Mapping {
                  lo: source,
                  hi: source + len,
                  tx: dest - source,
                })
              },
              // This actually means that the input is broken.
              | _ => None,
            }
          })
          .collect();

        Map { mappings }
      });

    let maps = map.separated_by(newline());

    let almanac = seeds
      .then_ignore(newline())
      .then(maps)
      .map(|(seeds, maps)| Almanac { seeds, maps });

    almanac.parse(s).map_err(|_| ParseError::InvalidInput)
  }
}

fn solve_part_one(input: &str) -> u32 {
  let almanac = Almanac::from_str(input).unwrap_or_default();

  let location = almanac
    .seeds
    .iter()
    .map(|seed| almanac.maps.iter().fold(*seed, |acc, map| map.tx(acc)))
    .min()
    .unwrap_or(0);

  location as u32
}

fn solve_part_two(input: &str) -> u32 {
  let almanac = Almanac::from_str(input).unwrap_or_default();

  let seeds = almanac
    .seeds
    .chunks(2)
    .map(|chunk| {
      let lo = chunk[0];
      let hi = chunk[0] + chunk[1];

      Seed { lo, hi }
    })
    .collect::<Vec<_>>();

  let location = almanac
    .maps
    .iter()
    .fold(seeds, |acc, map| map.txs(acc))
    .iter()
    .map(|x| x.lo)
    .min()
    .unwrap_or(0);

  location as u32
}

pub fn solution<'s>() -> Solution<'s, u32, u32> {
  Solution {
    title: "Day 5: If You Give A Seed A Fertilizer",
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
    assert_eq!(solve_part_one(EXAMPLE), 35);
    assert_eq!(solve_part_two(EXAMPLE), 46);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 227653707);
    assert_eq!(solve_part_two(INPUT), 78775051);
  }
}
