use super::models::*;

use futures::future;

const RELEASES_URL: &str = "https://api.opencritic.com/api/game?time=last90&sort=firstReleaseDate";
const PLATFORMS_URL: &str = "https://api.opencritic.com/api/platform";
const GENRES_URL: &str = "https://api.opencritic.com/api/genre";
const GAME_URL: &str = "https://api.opencritic.com/api/game";

pub fn get_platforms() -> Vec<Platform> {
  reqwest::blocking::get(PLATFORMS_URL)
    .unwrap()
    .json::<Vec<Platform>>()
    .unwrap()
}

pub fn get_genres() -> Vec<Genre> {
  reqwest::blocking::get(GENRES_URL)
    .unwrap()
    .json::<Vec<Genre>>()
    .unwrap()
}

pub async fn get_recent_releases(platforms: &[String], genres: &[String]) -> Vec<Game> {
  let releases = get_latest_releases().await;
  let ids = releases
    .iter()
    .map(|game| game.id)
    .collect::<Vec<i32>>();

  use chrono::Utc;
  let start = Utc::now();

  let mut games = future::join_all(
    ids
      .iter()
      .map(|&id| {
        async move {
          get_game_details(id).await
        }
      })
  ).await;

  let delta = Utc::now() - start;
  println!("it took {}ms", delta.num_milliseconds());

  filter_by_platforms(&mut games, platforms);
  filter_by_genres(&mut games, genres);

  games
}

pub async fn get_todays_releases(platforms: &[String], genres: &[String]) -> Vec<Game> {
  let mut games = get_recent_releases(platforms, genres).await;

  filter_by_date(&mut games);

  games
}

async fn get_latest_releases() -> Vec<BasicGameInfo> {
  reqwest::get(RELEASES_URL)
    .await
    .unwrap()
    .json::<Vec<BasicGameInfo>>()
    .await
    .unwrap()
}

async fn get_game_details(id: i32) -> Game {
  reqwest::get(format!("{}/{}", GAME_URL, id))
    .await
    .unwrap()
    .json::<Game>()
    .await
    .unwrap()
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
