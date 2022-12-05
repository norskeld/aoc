#![feature(box_syntax)]

use aoc::Printable;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
  let solutions: Vec<Box<dyn Printable>> = vec![
    box day_1::solution(),
    box day_2::solution(),
    box day_3::solution(),
    box day_4::solution(),
  ];

  aoc::print_solutions(&solutions);
}
