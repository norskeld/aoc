use aoc::Printable;

mod day_1;
mod day_2;

fn main() {
  let solutions: Vec<Box<dyn Printable>> =
    vec![Box::new(day_1::solution()), Box::new(day_2::solution())];

  aoc::print_solutions(&solutions);
}
