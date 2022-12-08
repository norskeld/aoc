//! Day 8: Treetop Tree House][link]
//!
//! [link]: https://adventofcode.com/2022/day/8

use aoc::{Part, Solution};

const INPUT: &str = include_str!("input.txt");

fn solve<const P: Part>(input: &str) -> usize {
  let lines = input.lines().collect::<Vec<_>>();

  let mut result = 0;
  let mut grid = Vec::with_capacity(lines.len());

  // Build the grid. The whole block looks wack as hell, should be rewritten.
  for row in 0..lines.len() {
    let trees = lines[row]
      .chars()
      .map(|ch| ch.to_digit(10).unwrap_or(0))
      .collect::<Vec<_>>();

    grid.push(Vec::with_capacity(trees.len()));

    for tree in 0..trees.len() {
      grid[row].push(trees[tree]);
    }
  }

  // Pre-define rows and cols amount.
  let rows = grid.len();
  let cols = grid.first().map(Vec::len).unwrap_or(0);

  // Kick the cringe off.
  for row in 0..rows {
    // Skip the first and the last rows, since they are always visible.
    if row == 0 || row == cols - 1 {
      continue;
    }

    // All rows have the same amount of cols.
    for col in 0..cols {
      // Also skip the first and the last cols.
      if col == 0 || col == cols - 1 {
        continue;
      }

      let height = grid[row][col];

      let mut left = true;
      let mut right = true;
      let mut top = true;
      let mut bottom = true;

      // Check the trees above the current one.
      for k_row in 0..row {
        if grid[k_row][col] >= height {
          top = false;
        }
      }

      // Check the trees to the left of the current one.
      for k_col in 0..col {
        if grid[row][k_col] >= height {
          left = false;
        }
      }

      // Check the trees below the current one.
      for k_row in row + 1..grid.len() {
        if grid[k_row][col] >= height {
          bottom = false;
        }
      }

      // Check the trees to the right of the current one.
      for k_col in col + 1..grid[row].len() {
        if grid[row][k_col] >= height {
          right = false;
        }
      }

      if left || right || top || bottom {
        result += 1;
      }
    }
  }

  // Calculate amount of trees on the edges to add to the result.
  let edges = 2 * (rows + cols) - 4;

  result + edges
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 8: Treetop Tree House",
    part_one: solve::<{ Part::One }>(INPUT),
    part_two: solve::<{ Part::Two }>(INPUT),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE: &str = include_str!("example.txt");

  #[test]
  fn test_examples() {
    assert_eq!(solve::<{ Part::One }>(EXAMPLE), 21);
    // assert_eq!(solve::<{ Part::Two }>(EXAMPLE), 0);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve::<{ Part::One }>(INPUT), 1703);
    // assert_eq!(solve::<{ Part::Two }>(INPUT), 0);
  }
}
