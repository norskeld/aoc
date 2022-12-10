//! [Day 10: Cathode-Ray Tube][link]
//!
//! [link]: https://adventofcode.com/2022/day/10

use std::str::FromStr;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum ParseError {
  UnknownInstruction,
  InvalidAddXValue,
  InvalidAddXInstruction,
}

#[derive(Clone, Debug)]
enum Instruction {
  Noop,
  Addx(isize),
}

impl FromStr for Instruction {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.starts_with("noop") {
      Ok(Instruction::Noop)
    } else if s.starts_with("addx") {
      match s.split_once(' ') {
        | Some((_, value)) => {
          value
            .parse::<isize>()
            .map(Instruction::Addx)
            .map_err(|_| ParseError::InvalidAddXValue)
        },
        | None => Err(ParseError::InvalidAddXInstruction),
      }
    } else {
      Err(ParseError::UnknownInstruction)
    }
  }
}

#[derive(Debug)]
struct Device {
  register: isize,
  cycle: usize,
  signal: isize,
  screen: Vec<String>,
}

impl Device {
  fn new() -> Self {
    Device {
      register: 1,
      cycle: 1,
      signal: 0,
      screen: Vec::with_capacity(240),
    }
  }

  fn tick(&mut self) {
    self.cycle += 1;
  }

  fn process(&mut self) {
    if self.cycle % 40 == 20 {
      self.signal += self.cycle as isize * self.register;
    }

    let draw_pos = (self.cycle as isize - 1) % 40;

    if (draw_pos - self.register).abs() <= 1 {
      self.screen.push("#".to_string());
    } else {
      self.screen.push(".".to_string());
    }
  }

  fn execute(&mut self, ins: &Instruction) {
    match ins {
      | Instruction::Noop => {
        self.process();
        self.tick();
      },
      | Instruction::Addx(value) => {
        self.process();
        self.tick();

        self.process();
        self.register += value;
        self.tick();
      },
    }
  }

  fn to_crt(&self) -> String {
    self
      .screen
      .chunks(40)
      .map(|chunks| chunks.to_owned().join(&String::default()))
      .collect::<Vec<String>>()
      .join("\n")
  }
}

fn solve_part_one(input: &str) -> isize {
  let mut device = Device::new();

  input
    .lines()
    .map(str::parse::<Instruction>)
    .map(Result::unwrap)
    .for_each(|instruction| device.execute(&instruction));

  device.signal
}

fn solve_part_two(input: &str) -> String {
  let mut device = Device::new();

  input
    .lines()
    .map(str::parse::<Instruction>)
    .map(Result::unwrap)
    .for_each(|instruction| device.execute(&instruction));

  device.to_crt()
}

pub fn solution<'s>() -> Solution<'s, isize, String> {
  Solution {
    title: "Day 10: Cathode-Ray Tube",
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
    let part_two = indoc::indoc! {"
      ##..##..##..##..##..##..##..##..##..##..
      ###...###...###...###...###...###...###.
      ####....####....####....####....####....
      #####.....#####.....#####.....#####.....
      ######......######......######......####
      #######.......#######.......#######.....
    "};

    assert_eq!(solve_part_one(EXAMPLE), 13140);
    assert_eq!(solve_part_two(EXAMPLE), part_two.trim());
  }

  #[test]
  fn test_input() {
    let part_two = indoc::indoc! {"
      ###..#..#.#....#..#...##..##..####..##..
      #..#.#..#.#....#..#....#.#..#....#.#..#.
      #..#.####.#....####....#.#......#..#..#.
      ###..#..#.#....#..#....#.#.##..#...####.
      #....#..#.#....#..#.#..#.#..#.#....#..#.
      #....#..#.####.#..#..##...###.####.#..#.
    "};

    assert_eq!(solve_part_one(INPUT), 15360);
    assert_eq!(solve_part_two(INPUT), part_two.trim());
  }
}
