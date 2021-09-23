use super::models::*;

const RELEASES_URL: &str = "https://api.opencritic.com/api/game?time=last90&sort=firstReleaseDate";
const PLATFORMS_URL: &str = "https://api.opencritic.com/api/platform";
const GENRES_URL: &str = "https://api.opencritic.com/api/genre";

struct OpenCriticApi;
impl OpenCriticApi {
  pub fn get_platforms() -> Vec<Platform> {
    todo!()
  }

  pub fn get_genres() -> Vec<Genre> {
    todo!()
  }

  pub fn get_todays_releases(platforms: &[String], genres: &[String]) -> Vec<Game> {
    todo!()
  }

  fn get_latest_releases() -> Vec<BasicGameInfo> {
    todo!()
  }

  fn get_game_details(id: i32) -> Game {
    todo!()
  }
}
