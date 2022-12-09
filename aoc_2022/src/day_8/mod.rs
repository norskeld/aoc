//! Day 8: Treetop Tree House][link]
//!
//! Playing with declarative macros, because why not. The overall perf and algo complexity is kinda
//! nasty, something like `O(n^3)`, but it's whatever, since it does the job in 3-4 ms on the big
//! puzzle input.
//!
//! [link]: https://adventofcode.com/2022/day/8

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

/// Simple macro for returning given result early if condition passes. Somewhat useful.
macro_rules! return_early_if {
  ($cond:expr, $result:expr) => {{
    if $cond {
      return $result;
    }
  }};
}

/// Another simple macro for continuing if condition passes. Very useful. /s
macro_rules! continue_if {
  ($cond:expr) => {
    if $cond {
      continue;
    }
  };
}

type Grid = Vec<Vec<usize>>;
type Pair<T> = (T, T);

fn parse_line(line: &str) -> Vec<usize> {
  line
    .chars()
    .map(|ch| ch.to_digit(10).unwrap_or(0) as usize)
    .collect()
}

fn prepare(input: &str) -> (Grid, usize, usize, usize) {
  let result = 0;

  // Build the grid.
  let grid = input.lines().map(parse_line).collect::<Grid>();

  // How much rows and cols? Will be needed later.
  let rows = grid.len();
  let cols = grid.first().map(Vec::len).unwrap_or(0);

  (grid, result, rows, cols)
}

fn check_visibility(grid: &Grid, height: usize, pos: Pair<usize>, bounds: Pair<usize>) -> bool {
  let (row, col) = pos;
  let (rows, cols) = bounds;

  // Check the trees above the current one.
  return_early_if!((0..row).all(|x| grid[x][col] < height), true);

  // Check the trees to the left of the current one.
  return_early_if!((0..col).all(|x| grid[row][x] < height), true);

  // Check the trees to the right of the current one.
  return_early_if!((col + 1..cols).all(|x| grid[row][x] < height), true);

  // Check the trees below the current one.
  return_early_if!((row + 1..rows).all(|x| grid[x][col] < height), true);

  false
}

fn scenic_score(grid: &Grid, height: usize, pos: Pair<usize>, bounds: Pair<usize>) -> usize {
  let (row, col) = pos;
  let (rows, cols) = bounds;

  let mut top = 0;
  let mut bottom = 0;
  let mut left = 0;
  let mut right = 0;

  for k in (0..row).rev() {
    top += 1;

    if grid[k][col] >= height {
      break;
    }
  }

  for k in (0..col).rev() {
    left += 1;

    if grid[row][k] >= height {
      break;
    }
  }

  for k in row + 1..rows {
    bottom += 1;

    if grid[k][col] >= height {
      break;
    }
  }

  for k in col + 1..cols {
    right += 1;

    if grid[row][k] >= height {
      break;
    }
  }

  left * right * top * bottom
}

fn solve_part_one(input: &str) -> usize {
  let (grid, mut result, rows, cols) = prepare(input);

  for row in 0..rows {
    // Skip the first and the last rows, since they are always visible.
    continue_if!(row == 0 || row == rows - 1);

    // All rows have the same amount of cols.
    for col in 0..cols {
      // Also skip the first and the last cols.
      continue_if!(col == 0 || col == cols - 1);

      // Height of the current tree.
      let height = grid[row][col];

      result += check_visibility(&grid, height, (row, col), (rows, cols)) as usize;
    }
  }

  // Calculate amount of trees on the edges to add to the result.
  let edges = 2 * (rows + cols) - 4;

  result + edges
}

fn solve_part_two(input: &str) -> usize {
  let (grid, mut result, rows, cols) = prepare(input);

  for row in 0..rows {
    // Skip the first and the last rows, since they are always visible.
    continue_if!(row == 0 || row == rows - 1);

    // All rows have the same amount of cols.
    for col in 0..cols {
      // Also skip the first and the last cols.
      continue_if!(col == 0 || col == cols - 1);

      // Height of the current tree.
      let height = grid[row][col];

      // Calculate scenic score for visible trees.
      let score = scenic_score(&grid, height, (row, col), (rows, cols));

      if score > result {
        result = score
      }
    }
  }

  result
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 8: Treetop Tree House",
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
    assert_eq!(solve_part_one(EXAMPLE), 21);
    assert_eq!(solve_part_two(EXAMPLE), 8);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 1703);
    assert_eq!(solve_part_two(INPUT), 496650);
  }
}
