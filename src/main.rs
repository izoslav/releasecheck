use clap::{AppSettings, Clap};
use serde::Deserialize;

use chrono::{DateTime, Utc};

const OPENCRITIC_URL: &str = "https://api.opencritic.com/api/game/upcoming";

#[derive(Debug, Deserialize)]
struct Company {
  name: String,
  r#type: String,
}

#[derive(Debug, Deserialize)]
struct Genre {
  name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Platform {
  name: String,
  short_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Game {
  // basic info
  name: String,
  first_release_date: DateTime<Utc>,
  #[serde(alias = "Companies")]
  companies: Vec<Company>,
  #[serde(alias = "Genres")]
  genres: Vec<Genre>,
  #[serde(alias = "Platforms")]
  platforms: Vec<Platform>,

  // score
  average_score: i32,
  tier: String,
}

impl Game {
  fn print(&self) {
    println!(
      "{} {} ({})",
      self.name,
      self.score(),
      self.platforms()
    );
  }

  fn score(&self) -> String {
    if self.average_score < 0 {
      "unscored".to_string()
    } else {
      format!("{} {}/100", self.tier, self.average_score)
    }
  }

  fn platforms(&self) -> String {
    self.platforms.iter()
      .map(|platform| {
        platform.short_name.clone()
      })
      .collect::<Vec<String>>()
      .join(", ")
  }
}

#[derive(Clap)]
#[clap(version = "0.1", author = "Marcin K. <dev@izoslav.pl>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Checks OpenCritic for games that were released today")]
struct Opts {
  #[clap(short, long, about = "Comma-separated list of platforms")]
  platforms: Option<String>,
  #[clap(short, long, about = "Prints all games returned by OpenCritic")]
  ignore_date: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let opts: Opts = Opts::parse();

  println!("platforms: {:?}", opts.platforms);
  println!("all_games: {}", opts.ignore_date);

  let games = reqwest::blocking::get(OPENCRITIC_URL)?
    .json::<Vec<Game>>()?;

  println!("games = {:?}", games);

  if games.len() == 0 {
    println!("No games released today :(");
    return Ok(());
  }

  for game in games {
    game.print();
  }

  Ok(())
}
