use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1", author = "Marcin K. <dev@izoslav.pl>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(short, long)]
  platforms: String,
  #[clap(short, long)]
  all_games: bool,
}

fn main() {
    println!("Hello, world!");
}
