//! [Day 3: Gear Ratios][link]
//!
//! Overengineered and iefficient solution with nasty complexity, but it is what it is.
//!
//! [link]: https://adventofcode.com/2023/day/3

use std::ops::Range;

use aoc::Solution;
use chumsky::prelude::*;

const INPUT: &str = include_str!("input.txt");

enum Check {
  Inclusive,
  Exclusive,
}

#[derive(Clone)]
enum Token {
  Number(u32, Range<usize>),
  Dot(Range<usize>),
  Symbol(char, Range<usize>),
}

impl Token {
  fn is_number(&self) -> bool {
    matches!(self, Token::Number(..))
  }

  fn is_symbol(&self) -> bool {
    matches!(self, Token::Symbol(..))
  }

  fn span(&self) -> &Range<usize> {
    match self {
      | Token::Dot(span) | Token::Number(_, span) | Token::Symbol(_, span) => span,
    }
  }

  fn with_span(&self, span: Range<usize>) -> Self {
    match self {
      | Token::Number(value, ..) => Token::Number(*value, span),
      | Token::Dot(..) => Token::Dot(span),
      | Token::Symbol(kind, ..) => Token::Symbol(*kind, span),
    }
  }

  fn overlaps_with(&self, tokens: &[Token], check: Check) -> bool {
    let span = self.span();

    tokens.iter().any(|token| {
      if token.is_symbol() {
        let other = token.span();

        match check {
          | Check::Inclusive => span.start < other.end && other.start <= span.end - 1,
          | Check::Exclusive => span.start < other.end && other.start < span.end - 1,
        }
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
fn has_adjacents_at(board: &Vec<Vec<Token>>, row: usize, token: &Token) -> bool {
  board.get(row).map_or(false, |tokens| {
    token.overlaps_with(tokens, Check::Exclusive)
  })
}

fn has_adjacents(board: &Vec<Vec<Token>>, row: usize, token: &Token) -> bool {
  let span = token.span();

  let col_min = span.start.saturating_sub(1);
  let col_max = span.end.saturating_add(2);

  let token = token.with_span(col_min..col_max);

  has_adjacents_at(&board, row, &token)
    || has_adjacents_at(&board, row + 1, &token)
    || has_adjacents_at(&board, row.saturating_sub(1), &token)
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
        if has_adjacents(&board, row, token) {
          result += value;
        }
      }
    }
  }

  result
}

fn find_gear_parts(board: &Vec<Vec<Token>>, row: usize, token: &Token) -> Option<(Token, Token)> {
  let span = token.span();

  // So we stumbled a star symbol and should find exactly 2 numbers adjacent to it. They can be
  // either in the current row or in the row above, or in the row below, so have to check them
  // one-by-one and stop when we have exactly 2 numbers found.

  // First, widen the search span so we can find numbers that lie diagonally.
  let col_min = span.start.saturating_sub(1);
  let col_max = span.end.saturating_add(1);

  let star = token.with_span(col_min..col_max);

  let mut first: Option<Token> = None;
  let mut second: Option<Token> = None;

  // Committing all possible crimes here, but whatever. Getting all possible tokens around the star.
  let grid = {
    let mut grid = Vec::new();

    if let Some(row) = board.get(row.saturating_sub(1)) {
      grid.extend(row.iter());
    }

    if let Some(row) = board.get(row) {
      grid.extend(row.iter());
    }

    if let Some(row) = board.get(row + 1) {
      grid.extend(row.iter());
    }

    grid
  };

  for token in grid.into_iter().filter(|token| token.is_number()) {
    if first.is_some() && second.is_some() {
      break;
    }

    if token.overlaps_with(&[star.clone()], Check::Inclusive) {
      if first.is_none() {
        first = Some(token.clone());
      } else if second.is_none() {
        second = Some(token.clone());
      }
    }
  }

  first.zip(second)
}

fn solve_part_two(input: &str) -> u32 {
  let mut board: Vec<Vec<Token>> = Vec::new();
  let mut vals: Vec<(Token, Token)> = Vec::new();
  let mut result = 0;

  for line in input.lines() {
    board.push(parse(line));
  }

  for (row, line) in board.iter().enumerate() {
    for token in line.iter() {
      if let Token::Symbol('*', ..) = token {
        if let Some(tokens) = find_gear_parts(&board, row, token) {
          vals.push(tokens);
        }
      }
    }
  }

  for pair in vals.iter() {
    if let (Token::Number(a, ..), Token::Number(b, ..)) = pair {
      result += a * b;
    }
  }

  result
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
    assert_eq!(solve_part_two(INPUT), 80703636);
  }
}
