use clap::{AppSettings, Clap};
use serde::Deserialize;

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
  name: String,
  first_release_date: String,
  #[serde(alias = "Companies")]
  companies: Vec<Company>,
  #[serde(alias = "Genres")]
  genres: Vec<Genre>,
  #[serde(alias = "Platforms")]
  platforms: Vec<Platform>,
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

  let body = reqwest::blocking::get(OPENCRITIC_URL)?
    .json::<Vec<Game>>()?;

  println!("body = {:?}", body);

  Ok(())
}
