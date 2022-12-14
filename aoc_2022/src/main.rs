#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(box_syntax)]

use aoc::Printable;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
  let solutions: Vec<Box<dyn Printable>> = vec![
    box day_1::solution(),
    box day_2::solution(),
    box day_3::solution(),
    box day_4::solution(),
    box day_5::solution(),
    box day_6::solution(),
    box day_7::solution(),
    box day_8::solution(),
    box day_9::solution(),
    box day_10::solution(),
    box day_11::solution(),
    box day_12::solution(),
  ];

  aoc::print_solutions(&solutions);
}
