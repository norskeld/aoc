//! [Day 12: Hill Climbing Algorithm][link]
//!
//! [link]: https://adventofcode.com/2022/day/12

use std::collections::VecDeque;
use std::ops::Range;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

type Grid = Vec<Vec<u8>>;
type Pair<T> = (T, T);

fn bfs(grid: &[Vec<u8>], start: &[(usize, usize)], goal: (usize, usize)) -> Option<usize> {
  let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

  let mut queue = start
    .iter()
    .map(|&(x, y)| (x, y, 0))
    .collect::<VecDeque<_>>();

  while let Some((x, y, len)) = queue.pop_front() {
    if (x, y) == goal {
      return Some(len);
    }

    for (dx, dy) in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
      let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
      let Some(&square) = grid.get(nx).and_then(|row| row.get(ny)) else {
        continue;
      };

      if grid[x][y] + 1 >= square && !visited[nx][ny] {
        visited[nx][ny] = true;
        queue.push_back((nx, ny, len + 1));
      }
    }
  }

  None
}

fn parse_grid(input: &str) -> Grid {
  input
    .lines()
    .map(|line| line.as_bytes().to_vec())
    .collect::<Vec<_>>()
}

fn cartesian_product(rows: Range<usize>, cols: Range<usize>) -> impl Iterator<Item = Pair<usize>> {
  rows.flat_map(move |row| cols.clone().map(move |col| (row, col)))
}

fn grid_sizes(grid: &Grid) -> Pair<Range<usize>> {
  let rows = 0..grid.len();
  let cols = 0..grid[0].len();

  (rows, cols)
}

fn prepare_grid(mut grid: Grid) -> (Grid, Pair<usize>, Pair<usize>) {
  // let rows = 0..grid.len();
  // let cols = 0..grid[0].len();

  let (rows, cols) = grid_sizes(&grid);

  // Find `S`tart position.
  let (sx, sy) = cartesian_product(rows.clone(), cols.clone())
    .find(|(x, y)| grid[*x][*y] == b'S')
    .unwrap();

  // Find `E`nd position.
  let (ex, ey) = cartesian_product(rows, cols)
    .find(|(x, y)| grid[*x][*y] == b'E')
    .unwrap();

  grid[sx][sy] = b'a';
  grid[ex][ey] = b'z';

  (grid, (sx, sy), (ex, ey))
}

fn solve_part_one(input: &str) -> usize {
  let grid = parse_grid(input);
  let (grid, start, end) = prepare_grid(grid);

  bfs(&grid, &[start], end).unwrap()
}

fn solve_part_two(input: &str) -> usize {
  let grid = parse_grid(input);
  let (grid, _, (gx, gy)) = prepare_grid(grid);
  let (rows, cols) = grid_sizes(&grid);

  let positions = cartesian_product(rows, cols)
    .filter(|&(x, y)| grid[x][y] == b'a')
    .collect::<Vec<_>>();

  bfs(&grid, &positions, (gx, gy)).unwrap()
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 12: Hill Climbing Algorithm",
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
    assert_eq!(solve_part_one(EXAMPLE), 31);
    assert_eq!(solve_part_two(EXAMPLE), 29);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve_part_one(INPUT), 449);
    assert_eq!(solve_part_two(INPUT), 443);
  }
}
