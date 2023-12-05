use std::fs;
use std::io;
use std::path::PathBuf;

use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use indoc::indoc;

const MOD_TEMPLATE: &str = indoc! {r#"
  //! [Day {day}: {day_title}][link]
  //!
  //! [link]: https://adventofcode.com/{year}/day/{day}

  use aoc::Solution;

  fn solve_part_one(_input: &str) -> u32 {
    0
  }

  fn solve_part_two(_input: &str) -> u32 {
    0
  }

  pub fn solution<'s>() -> Solution<'s, u32, u32> {
    Solution {
      title: "Day {day}: {day_title}",
      part_one: solve_part_one(INPUT),
      part_two: solve_part_two(INPUT),
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_examples() {
      // assert_eq!(solve_part_one(EXAMPLE), 0);
      // assert_eq!(solve_part_two(EXAMPLE), 0);
    }

    #[test]
    fn test_input() {
      // assert_eq!(solve_part_one(INPUT), 0);
      // assert_eq!(solve_part_two(INPUT), 0);
    }
  }
"#};

#[derive(Debug)]
struct Paths {
  root: PathBuf,
  module: PathBuf,
  example: PathBuf,
  input: PathBuf,
}

#[derive(Debug)]
struct Day {
  year: String,
  day: String,
  title: String,
}

impl Day {
  fn from(year: String, day: String, title: String) -> Self {
    Self { year, day, title }
  }

  fn paths(&self) -> Paths {
    let root = format!("aoc_{year}/src/day_{day}", year = self.year, day = self.day);

    let module = format!("{root}/mod.rs");
    let example = format!("{root}/example.txt");
    let input = format!("{root}/input.txt");

    Paths {
      root: PathBuf::from(root),
      module: PathBuf::from(module),
      example: PathBuf::from(example),
      input: PathBuf::from(input),
    }
  }

  fn apply_to(&self, template: &str) -> String {
    template
      .replace("{year}", &self.year)
      .replace("{day}", &self.day)
      .replace("{day_title}", &self.title)
  }

  fn write(&self) -> io::Result<()> {
    let paths = self.paths();
    let template = self.apply_to(MOD_TEMPLATE);

    fs::create_dir_all(&paths.root)?;

    fs::write(&paths.module, template)?;
    fs::write(&paths.example, "")?;
    fs::write(&paths.input, "")?;

    Ok(())
  }
}

pub fn run() {
  let year: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Year")
    .interact_text()
    .expect("failed to read year");

  let day: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Day")
    .interact_text()
    .expect("failed to read day");

  let title: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Title")
    .interact_text()
    .expect("failed to read day title");

  let day = Day::from(year, day, title);

  match day.write() {
    | Ok(_) => {
      println!("  ———");
      println!("· Done!");
    },
    | Err(err) => {
      eprintln!("  ———");
      eprintln!("· Failed!");
      eprintln!("    Details: {err:?}");
    },
  }
}
