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
    Box::new(day_1::solution()),
    Box::new(day_2::solution()),
    Box::new(day_3::solution()),
    Box::new(day_4::solution()),
    Box::new(day_5::solution()),
    Box::new(day_6::solution()),
    Box::new(day_7::solution()),
    Box::new(day_8::solution()),
    Box::new(day_9::solution()),
    Box::new(day_10::solution()),
    Box::new(day_11::solution()),
    Box::new(day_12::solution()),
  ];

  aoc::print_solutions(&solutions);
}
