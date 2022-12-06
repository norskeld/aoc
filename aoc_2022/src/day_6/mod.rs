//! [Day 6: Tuning Trouble][link]
//!
//! [link]: https://adventofcode.com/2022/day/6

use std::str;

use aoc::Solution;

const INPUT: &str = include_str!("input.txt");

fn solve<const N: usize>(s: &str) -> usize {
  s.as_bytes()
    .windows(N)
    .position(|chunk| {
      (1..chunk.len()).all(move |cursor| !chunk[cursor..].contains(&chunk[cursor - 1]))
    })
    .map(|pos| pos + N)
    .unwrap_or_default()
}

pub fn solution<'s>() -> Solution<'s, usize, usize> {
  Solution {
    title: "Day 6: Tuning Trouble",
    part_one: solve::<4>(INPUT),
    part_two: solve::<14>(INPUT),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_examples() {
    // Part 1.
    assert_eq!(solve::<4>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(solve::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(solve::<4>("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(solve::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(solve::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

    // Part 2.
    assert_eq!(solve::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(solve::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(solve::<14>("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(solve::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(solve::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
  }

  #[test]
  fn test_input() {
    assert_eq!(solve::<4>(INPUT), 1850);
    assert_eq!(solve::<14>(INPUT), 2823);
  }
}
