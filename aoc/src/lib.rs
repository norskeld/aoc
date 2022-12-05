use std::fmt::Display;

use indoc::printdoc;

pub trait Printable {
  fn print(&self);
}

pub struct Solution<'s, O: Display, T: Display> {
  pub title: &'s str,
  pub part_one: O,
  pub part_two: T,
}

impl<'s, O: Display, T: Display> Printable for Solution<'s, O, T> {
  fn print(&self) {
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

pub fn print_solutions(solutions: &[Box<dyn Printable>]) {
  for solution in solutions {
    solution.print();
  }
}
