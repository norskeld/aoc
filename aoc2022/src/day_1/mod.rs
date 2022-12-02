const INPUT: &'static str = include_str!("input.txt");

struct Calories<const N: usize> {
  values: [u64; N],
}

impl<const N: usize> Calories<N> {
  const fn new() -> Self {
    Self { values: [0; N] }
  }

  fn update(&mut self, next: u64) {
    self.values.sort_unstable();

    for current in self.values.iter_mut() {
      if *current < next {
        *current = next;
        return;
      }
    }
  }

  fn sum(&self) -> u64 {
    self.values.iter().sum()
  }
}

impl<const N: usize> FromIterator<u64> for Calories<N> {
  fn from_iter<I>(it: I) -> Self
  where
    I: IntoIterator<Item = u64>,
  {
    it.into_iter().fold(Self::new(), |mut acc, value| {
      acc.update(value);
      acc
    })
  }
}

pub fn solve_for<const N: usize>(input: &str) -> u64 {
  // Looks dirty, but works.
  input
    .split("\n\n")
    .map(|elf| {
      elf
        .lines()
        .map(|line| u64::from(line.parse::<u32>().unwrap_or_default()))
        .sum()
    })
    // Just playing around with const generics and iterators...
    .collect::<Calories<N>>()
    .sum()
}

pub fn solution() {
  let part_one = solve_for::<1>(INPUT);
  let part_two = solve_for::<3>(INPUT);

  println!("part one: {part_one}");
  println!("part two: {part_two}");
}
