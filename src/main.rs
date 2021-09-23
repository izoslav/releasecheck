use clap::{AppSettings, Clap};
use serde::Deserialize;

use chrono::{Datelike, DateTime, Utc};

const OPENCRITIC_URL: &str = "https://api.opencritic.com/api/game/recently-released";

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
  average_score: f32,
  tier: String,
}

impl Game {
  fn print(&self) {
    println!(
      "{} - {} - {}",
      self.name,
      self.score(),
      self.platforms()
    );
  }

  fn score(&self) -> String {
    if self.average_score < 0. {
      "unscored".to_string()
    } else {
      format!("{} {:.0}/100", self.tier, self.average_score)
    }
  }

  fn platforms(&self) -> String {
    self.platforms.iter()
      .map(|platform| platform.short_name.clone())
      .collect::<Vec<String>>()
      .join(", ")
  }

  fn released_for(&self, platforms: &Vec<String>) -> bool {
    if platforms.len() > 0 {
      self.platforms.iter()
        .any(|platform|
          platforms.contains(&platform.short_name)
        )
    } else {
      true
    }
  }

  fn released_today(&self) -> bool {
    let now = Utc::now();
    let release = self.first_release_date;

    release.year() == now.year() &&
    release.month() == now.month() &&
    release.day() == now.day()
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

  let platforms = opts.platforms
    .as_ref()
    .map_or_else(
      || Vec::new(),
      |p| p.split(",").map(|e| e.to_string()).collect());

  println!("{:?}", platforms);

  let mut games = reqwest::blocking::get(OPENCRITIC_URL)?
    .json::<Vec<Game>>()?;
  games.sort_by(|a, b| a.name.cmp(&b.name));

  // println!("games = {:?}", games);

  if games.len() == 0 {
    println!("No games released today :(");
    return Ok(());
  }

  let games = games
    .iter()
    .filter(|&game| {
      let released_today = opts.ignore_date || game.released_today();
      
      released_today && game.released_for(&platforms)
    })
    .collect::<Vec<&Game>>();

  if games.len() == 0 {
    println!(
      "No relevant games released {}:(",
      if opts.ignore_date { "" } else { "today " }
    );
  } else {
    for game in games {
      game.print();
    }
  }

  Ok(())
}
