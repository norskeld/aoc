mod day_1;
mod day_2;

use std::fmt::Display;

use indoc::printdoc;

pub struct Solution<'s, O, T> {
  pub title: &'s str,
  pub part_one: O,
  pub part_two: T,
}

impl<'s, O, T> Solution<'s, O, T>
where
  O: Display,
  T: Display,
{
  pub fn print(&self) {
    let Solution {
      title,
      part_one,
      part_two,
    } = self;

    printdoc! {"
      {title}

      - Part 1: {part_one}
      - Part 2: {part_two}

    "};
  }
}

fn main() {
  let solutions = vec![day_1::solution(), day_2::solution()];

  for solution in solutions.iter() {
    solution.print();
  }
}
