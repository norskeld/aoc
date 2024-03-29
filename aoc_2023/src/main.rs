use aoc::Printable;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn main() {
  let solutions: Vec<Box<dyn Printable>> = vec![
    Box::new(day_1::solution()),
    Box::new(day_2::solution()),
    Box::new(day_3::solution()),
    Box::new(day_4::solution()),
    Box::new(day_5::solution()),
    Box::new(day_6::solution()),
  ];

  aoc::print_solutions(&solutions);
}
