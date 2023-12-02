use aoc::Printable;

mod day_1;

fn main() {
  let solutions: Vec<Box<dyn Printable>> = vec![Box::new(day_1::solution())];

  aoc::print_solutions(&solutions);
}
