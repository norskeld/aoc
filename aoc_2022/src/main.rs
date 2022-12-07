#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(box_syntax)]

use aoc::Printable;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

fn main() {
  let solutions: Vec<Box<dyn Printable>> = vec![
    box day_1::solution(),
    box day_2::solution(),
    box day_3::solution(),
    box day_4::solution(),
    box day_5::solution(),
    box day_6::solution(),
    box day_7::solution(),
  ];

  aoc::print_solutions(&solutions);
}
