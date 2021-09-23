#[macro_use] extern crate prettytable;

mod opencritic;

use opencritic::models::*;

use clap::{AppSettings, Clap};
use prettytable::Table;

use std::error::Error;

const OPENCRITIC_RELEASES_URL: &str = "https://api.opencritic.com/api/game/recently-released";
const OPENCRITIC_PLATFORMS_URL: &str = "https://api.opencritic.com/api/platform";
const OPENCRITIC_GENRES_URL: &str = "https://api.opencritic.com/api/genre";

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
  #[clap(long, about = "List available genres, and exits program")]
  list_genres: bool,
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

  println!("Available platforms:");
  table.printstd();

  Ok(())
}

fn list_genres() -> Result<(), Box<dyn Error>> {
  let mut genres = reqwest::blocking::get(OPENCRITIC_GENRES_URL)?
    .json::<Vec<Genre>>()?;

  genres.sort_by(|a, b| a.name.cmp(&b.name));

  println!("Available genres:");

  for genre in genres {
    println!("- {}", genre.name);
  }

  Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
  let opts: Opts = Opts::parse();

  if opts.list_platforms {
    return list_platforms();
  }

  if opts.list_genres {
    return list_genres();
  }

  let platforms = opts.platforms
    .as_ref()
    .map_or_else(
      Vec::new,
      |p| p.split(',').map(|e| e.to_string()).collect()
    );

  let genres = opts.genres
    .as_ref()
    .map_or_else(
      Vec::new,
      |g| g.split(',').map(|e| e.to_string().to_lowercase()).collect()
    );

  let mut games = reqwest::blocking::get(OPENCRITIC_RELEASES_URL)?
    .json::<Vec<Game>>()?;
  games.sort_by(|a, b| a.name.cmp(&b.name));

  if !opts.ignore_date { filter_by_date(&mut games); }
  filter_by_platforms(&mut games, &platforms);
  filter_by_genres(&mut games, &genres);

  if games.is_empty() {
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

fn filter_by_platforms(games: &mut Vec<Game>, platforms: &[String]) {
  if !platforms.is_empty() {
    games.retain(|game| game.released_for(platforms));
  }
}

fn filter_by_genres(games: &mut Vec<Game>, genres: &[String]) {
  if !genres.is_empty() {
    games.retain(|game| {
      genres.iter().any(|genre| {
        game.genres().to_lowercase().contains(genre)
      })
    })
  }
}
