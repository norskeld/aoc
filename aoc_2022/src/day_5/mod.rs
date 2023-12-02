//! [Day 5: Supply Stacks][link]
//!
//! [link]: https://adventofcode.com/2022/day/5

use std::mem;
use std::str;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

/// Represents a stack of crates.
type Stack = Vec<char>;

#[derive(Debug, Default)]
struct Crates {
  table: Vec<Stack>,
}

impl Crates {
  fn from_line(mut crates: Crates, line: &str) -> Crates {
    line
      .as_bytes()
      .chunks(4)
      .map(str::from_utf8)
      .map(Result::unwrap)
      .map(|chunk| {
        chunk
          .trim()
          .chars()
          .nth(1)
          .filter(char::is_ascii_alphabetic)
      })
      .enumerate()
      .for_each(|(index, value)| {
        crates
          .table
          .resize_with(crates.table.len().max(index + 1), Default::default);

        crates.table[index].extend(value)
      });

    crates
  }

  fn run(&mut self, cmd: Move) -> &mut Self {
    let Move { from, to, stack } = cmd;

    let from = from - 1;
    let to = to - 1;

    for _ in 0..stack {
      let elem = self.table[from].pop();
      self.table[to].extend(elem)
    }

    self
  }

  fn run_preserving(&mut self, cmd: Move) -> &mut Self {
    let Move { from, to, stack } = cmd;

    let from = from - 1;
    let to = to - 1;

    let mut next_stack = mem::take(&mut self.table[to]);

    let crates = {
      let current_stack = &mut self.table[from];
      let drain_range = (current_stack.len() - stack)..;

      current_stack.drain(drain_range)
    };

    next_stack.extend(crates);

    self.table[to] = next_stack;

    self
  }

  fn top(&self) -> String {
    self.table.iter().flat_map(|stack| stack.last()).collect()
  }
}

struct Move {
  stack: usize,
  from: usize,
  to: usize,
}

impl Move {
  fn from_line(line: &str) -> Move {
    let values: Vec<_> = line.split_whitespace().collect();

    if let ["move", stack, "from", from, "to", to] = *values.as_slice() {
      let [stack, from, to] = [stack, from, to]
        .map(str::parse::<usize>)
        .map(Result::unwrap);

      Move { stack, from, to }
    } else {
      panic!("Couldn't parse the move command.")
    }
  }
}

fn solve_common(s: &str) -> (Crates, Vec<Move>) {
  let mut lines = s.lines();

  let mut crates = lines
    .by_ref()
    .take_while(|s| !s.is_empty())
    .fold(Crates::default(), Crates::from_line);

  crates.table.iter_mut().for_each(|stack| stack.reverse());

  let moves = lines
    .filter(|s| !s.is_empty())
    .map(Move::from_line)
    .collect::<Vec<_>>();

  (crates, moves)

  // match P {
  //   | Part::One => it.by_ref().fold(&mut crates, Crates::run).top(),
  //   | Part::Two => it.by_ref().fold(&mut crates, Crates::run_preserving).top(),
  // }
}

fn solve_part_one(s: &str) -> String {
  let (mut crates, moves) = solve_common(s);

  moves.into_iter().fold(&mut crates, Crates::run).top()
}

fn solve_part_two(s: &str) -> String {
  let (mut crates, moves) = solve_common(s);

  moves
    .into_iter()
    .fold(&mut crates, Crates::run_preserving)
    .top()
}

pub fn solution<'s>() -> Solution<'s, String, String> {
  Solution {
    title: "Day 5: Supply Stacks",
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
    assert_eq!(solve_part_one(EXAMPLE), "CMZ".to_string());
    assert_eq!(solve_part_two(EXAMPLE), "MCD".to_string());
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), "ZRLJGSCTR".to_string());
    assert_eq!(solve_part_two(INPUT), "PRTTGRFPB".to_string());
  }
}
