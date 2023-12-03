//! [Day 3: Gear Ratios][link]
//!
//! Overengineered solution with nasty complexity, but it is what it is.
//!
//! [link]: https://adventofcode.com/2023/day/3

use std::ops::Range;

use aoc::Solution;
use chumsky::prelude::*;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Token {
  Number(u32, Range<usize>),
  Dot(Range<usize>),
  Symbol(char, Range<usize>),
}

impl Token {
  fn span(&self) -> &Range<usize> {
    match self {
      | Token::Dot(span) | Token::Number(.., span) | Token::Symbol(.., span) => span,
    }
  }

  fn with_span(&self, span: Range<usize>) -> Self {
    match self {
      | Token::Number(value, ..) => Token::Number(*value, span),
      | Token::Dot(..) => Token::Dot(span),
      | Token::Symbol(kind, ..) => Token::Symbol(*kind, span),
    }
  }

  fn overlaps_with(&self, tokens: &[Token]) -> bool {
    let span = self.span();

    tokens.into_iter().any(|token| {
      if matches!(token, Token::Symbol { .. }) {
        let other = token.span();

        span.start < other.end && span.end - 1 > other.start
      } else {
        false
      }
    })
  }
}

fn parse(line: &str) -> Vec<Token> {
  let number = text::digits::<_, Simple<char>>(10)
    .map_with_span(|digits, span| Token::Number(digits.parse().unwrap_or(0), span));

  let dot = just::<_, _, Simple<char>>('.').map_with_span(|_, span| Token::Dot(span));

  let symbol = choice::<_, Simple<char>>((dot, number))
    .not()
    .map_with_span(|kind, span| Token::Symbol(kind, span));

  let parser = choice::<_, Simple<char>>((number, dot, symbol))
    .repeated()
    .collect::<Vec<Token>>();

  parser.parse(line).unwrap_or(vec![])
}

#[inline]
fn has_neighbors_at(board: &Vec<Vec<Token>>, row: usize, token: &Token) -> bool {
  board
    .get(row)
    .map_or(false, |tokens| token.overlaps_with(tokens))
}

fn has_neighbors(board: &Vec<Vec<Token>>, row: usize, token: &Token) -> bool {
  let span = token.span();

  let col_min = span.start.saturating_sub(1);
  let col_max = span.end.saturating_add(2);

  let token = token.with_span(col_min..col_max);

  has_neighbors_at(&board, row, &token)
    || has_neighbors_at(&board, row + 1, &token)
    || has_neighbors_at(&board, row.saturating_sub(1), &token)
}

fn solve_part_one(input: &str) -> u32 {
  let mut board: Vec<Vec<Token>> = Vec::new();
  let mut result = 0;

  for line in input.lines() {
    board.push(parse(line));
  }

  for (row, line) in board.iter().enumerate() {
    for token in line.iter() {
      if let Token::Number(value, ..) = token {
        if has_neighbors(&board, row, token) {
          result += value;
        }
      }
    }
  }

  result
}

fn solve_part_two(_input: &str) -> u32 {
  0
}

pub fn solution<'s>() -> Solution<'s, u32, u32> {
  Solution {
    title: "Day 3: Gear Ratios",
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
    assert_eq!(solve_part_one(EXAMPLE), 4361);
    assert_eq!(solve_part_two(EXAMPLE), 467835);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 539590);
    // assert_eq!(solve_part_two(INPUT), 69110);
  }
}
