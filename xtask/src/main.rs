mod day;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, long_about = None)]
struct Cli {
  #[command(subcommand)]
  task: Option<Task>,
}

#[derive(Subcommand)]
enum Task {
  Day,
}

fn main() {
  let cli = Cli::parse();

  if let Some(task) = &cli.task {
    match task {
      | Task::Day => day::run(),
    }
  } else {
    println!("No task specified.");
  }
}
