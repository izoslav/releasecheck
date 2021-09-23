#[macro_use] extern crate prettytable;

use clap::{AppSettings, Clap};
use prettytable::Table;
use serde::Deserialize;

use std::error::Error;
use chrono::{Datelike, DateTime, Utc};

const OPENCRITIC_RELEASES_URL: &str = "https://api.opencritic.com/api/game/recently-released";
const OPENCRITIC_PLATFORMS_URL: &str = "https://api.opencritic.com/api/platform";

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

  fn genres(&self) -> String {
    self.genres.iter()
      .map(|genre| genre.name.clone())
      .collect::<Vec<String>>()
      .join(", ")
  }

  fn released_for(&self, platforms: &Vec<String>) -> bool {
    self.platforms.iter()
      .any(|platform|
        platforms.contains(&platform.short_name)
      )
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
  // options
  #[clap(short, long, about = "Prints all games returned by OpenCritic")]
  ignore_date: bool,
  // filters
  #[clap(short, long, about = "Comma-separated list of platform short names for filtering")]
  platforms: Option<String>,
  #[clap(short, long, about = "Comma-separated list of genres for filtering")]
  genres: Option<String>,
  // subcommands
  #[clap(long, about = "List available platforms and their short names, and exits program")]
  list_platforms: bool,
}

fn list_platforms() -> Result<(), Box<dyn Error>> {
  let mut platforms = reqwest::blocking::get(OPENCRITIC_PLATFORMS_URL)?
    .json::<Vec<Platform>>()?;

  platforms.sort_by(|a, b| a.name.cmp(&b.name));

  let mut table = Table::new();
  table.set_titles(row!["Name", "Short name"]);

  for platform in platforms {
    table.add_row(row![platform.name, platform.short_name]);
  }

  table.printstd();

  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  let opts: Opts = Opts::parse();

  if opts.list_platforms {
    return list_platforms();
  }

  let platforms = opts.platforms
    .as_ref()
    .map_or_else(
      || Vec::new(),
      |p| p.split(",").map(|e| e.to_string()).collect()
    );

  let genres = opts.genres
    .as_ref()
    .map_or_else(
      || Vec::new(),
      |g| g.split(",").map(|e| e.to_string().to_lowercase()).collect()
    );

  let mut games = reqwest::blocking::get(OPENCRITIC_RELEASES_URL)?
    .json::<Vec<Game>>()?;
  games.sort_by(|a, b| a.name.cmp(&b.name));

  if games.len() == 0 {
    println!("No games released today :(");
    return Ok(());
  }

  if !opts.ignore_date { filter_by_date(&mut games); }
  filter_by_platforms(&mut games, &platforms);
  filter_by_genres(&mut games, &genres);

  if games.len() == 0 {
    println!(
      "🔴 No relevant games released {} 😢",
      if opts.ignore_date { "recently" } else { "today" }
    );
  } else {
    println!("📀 Today's releases:");

    let mut table = Table::new();
    table.set_titles(row!["Name", "Score", "Genres", "Platforms"]);

    for game in games {
      table.add_row(row![
        game.name,
        game.score(),
        game.genres(),
        game.platforms(),
      ]);
    }

    table.printstd();
  }

  Ok(())
}

fn filter_by_date(games: &mut Vec<Game>) {
  games.retain(|game| game.released_today())
}

fn filter_by_platforms(games: &mut Vec<Game>, platforms: &Vec<String>) {
  if platforms.len() > 0 {
    games.retain(|game| game.released_for(&platforms));
  }
}

fn filter_by_genres(games: &mut Vec<Game>, genres: &Vec<String>) {
  if genres.len() > 0 {
    games.retain(|game| {
      genres.iter().any(|genre| {
        game.genres().to_lowercase().contains(genre)
      })
    })
  }
}
