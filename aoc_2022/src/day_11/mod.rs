//! [Day 11: Monkey in the Middle][link]
//!
//! [link]: https://adventofcode.com/2022/day/11

use std::str::FromStr;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum ParseError {
  Line,
  Operation,
  OperationValue,
  TestBy,
  IfValue,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
  Add(usize),
  Multiply(usize),
  MultiplySelf,
}

impl Operation {
  fn perform(&self, x: usize) -> usize {
    match self {
      | Operation::Add(y) => x + y,
      | Operation::Multiply(y) => x * y,
      | Operation::MultiplySelf => x * x,
    }
  }
}

impl FromStr for Operation {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts = s
      .split_ascii_whitespace()
      .map(str::trim)
      .collect::<Vec<_>>();

    match parts.as_slice() {
      | ["old", "*", "old"] => Ok(Operation::MultiplySelf),
      | ["old", "*", n] => {
        n.parse::<usize>()
          .map(Operation::Multiply)
          .map_err(|_| ParseError::OperationValue)
      },
      | ["old", "+", n] => {
        n.parse::<usize>()
          .map(Operation::Add)
          .map_err(|_| ParseError::OperationValue)
      },
      | _ => Err(ParseError::Operation),
    }
  }
}

#[derive(Clone, Debug)]
struct Monkey {
  /// Starting items.
  items: Vec<usize>,
  /// Operation to perform.
  operation: Operation,
  /// Divisibility test.
  test_mod: usize,
  /// Monkey ID if test is `true`.
  if_true: usize,
  /// Monkey ID if test is `false`.
  if_false: usize,
  /// How many times monkey inspected items.
  inspections: usize,
}

impl Monkey {
  fn from_lines(lines: &[&str], current: &mut usize) -> Result<Monkey, ParseError> {
    // Parse `Starting items`.

    let items = lines[*current + 1]
      .trim()
      .split_once(':')
      .ok_or(ParseError::Line)?;

    let items = match items {
      | (_, values) => {
        values
          .split(',')
          .map(str::trim)
          .filter_map(|value| value.parse::<usize>().ok())
          .collect::<Vec<_>>()
      },
    };

    // Parse `Operation`.

    let operation = &lines[*current + 2]
      .trim()
      .split_once('=')
      .ok_or(ParseError::Line)?;

    let operation = match operation {
      | (_, binop) => binop.trim().parse::<Operation>()?,
    };

    // Parse `Test`.

    let test_mod = lines[*current + 3]
      .trim()
      .split_once("by")
      .ok_or(ParseError::Line)?;

    let test_mod = match test_mod {
      | (_, divisible_by) => {
        divisible_by
          .trim()
          .parse::<usize>()
          .map_err(|_| ParseError::TestBy)?
      },
    };

    // Parse `If true`.

    let if_true = lines[*current + 4]
      .trim()
      .split_once("monkey")
      .ok_or(ParseError::Line)?;

    let if_true = match if_true {
      | (_, value) => {
        value
          .trim()
          .parse::<usize>()
          .map_err(|_| ParseError::IfValue)?
      },
    };

    // Parse `If false`.

    let if_false = lines[*current + 5]
      .trim()
      .split_once("monkey")
      .ok_or(ParseError::Line)?;

    let if_false = match if_false {
      | (_, value) => {
        value
          .trim()
          .parse::<usize>()
          .map_err(|_| ParseError::IfValue)?
      },
    };

    *current += 7;

    Ok(Monkey {
      items,
      operation,
      test_mod,
      if_true,
      if_false,
      inspections: 0,
    })
  }
}

fn parse_monkeys(lines: &[&str]) -> Result<Vec<Monkey>, ParseError> {
  let mut monkeys = Vec::new();
  let mut current = 0;

  while current < lines.len() {
    monkeys.push(Monkey::from_lines(lines, &mut current)?);
  }

  Ok(monkeys)
}

fn solve_part_one(input: &str) -> usize {
  const ROUNDS: usize = 20;

  let lines = input.lines().collect::<Vec<_>>();
  let mut monkeys = parse_monkeys(&lines).unwrap();

  for _ in 0..ROUNDS {
    for monkey_index in 0..monkeys.len() {
      while !monkeys[monkey_index].items.is_empty() {
        monkeys[monkey_index].inspections += 1;

        let item = monkeys[monkey_index].items.remove(0);
        let item = monkeys[monkey_index].operation.perform(item) / 3;

        let next_monkey = if item % monkeys[monkey_index].test_mod == 0 {
          monkeys[monkey_index].if_true
        } else {
          monkeys[monkey_index].if_false
        };

        monkeys[next_monkey].items.push(item);
      }
    }
  }

  let mut inspected_counts = monkeys
    .iter()
    .map(|x| x.inspections)
    .collect::<Vec<usize>>();

  // Sort highest to lowest.
  inspected_counts.sort_by(|a, b| b.cmp(a));

  // Return the product of the top two numbers.
  inspected_counts.into_iter().take(2).product()
}

fn solve_part_two(input: &str) -> usize {
  const ROUNDS: usize = 10000;

  let lines = input.lines().collect::<Vec<_>>();
  let mut monkeys = parse_monkeys(&lines).unwrap();

  // Working off a modulus of the product of all the test_mods allows
  // for the operations to be congruent to the mod of all the monkey test_mods
  let mod_product = monkeys.iter().map(|x| x.test_mod).product::<usize>();

  for _ in 0..ROUNDS {
    for index in 0..monkeys.len() {
      while !monkeys[index].items.is_empty() {
        monkeys[index].inspections += 1;

        let mut item = monkeys[index].items.remove(0);
        item = monkeys[index].operation.perform(item) % mod_product;

        let next_monkey = if item % monkeys[index].test_mod == 0 {
          monkeys[index].if_true
        } else {
          monkeys[index].if_false
        };

        monkeys[next_monkey].items.push(item);
      }
    }
  }

  let mut inspected_counts: Vec<usize> = monkeys.iter().map(|x| x.inspections).collect();

  // Sort highest to lowest.
  inspected_counts.sort_by(|a, b| b.cmp(a));

  // Return the product of the top two numbers.
  inspected_counts.into_iter().take(2).product()
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 11: Monkey in the Middle",
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
    assert_eq!(solve_part_one(EXAMPLE), 10605);
    assert_eq!(solve_part_two(EXAMPLE), 2713310158);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 69918);
    assert_eq!(solve_part_two(INPUT), 19573408701);
  }
}
