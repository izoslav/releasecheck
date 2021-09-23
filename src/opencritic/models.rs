use chrono::{DateTime, Datelike, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    pub name: String,
    pub short_name: String,
}

#[derive(Debug, Deserialize)]
pub struct BasicGameInfo {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    // basic info
    pub name: String,
    first_release_date: DateTime<Local>,
    #[serde(alias = "Genres")]
    genres: Vec<Genre>,
    #[serde(alias = "Platforms")]
    platforms: Vec<Platform>,

    // score
    average_score: f32,
    tier: String,
}

impl Game {
    pub fn score(&self) -> String {
        if self.average_score < 0. {
            "unscored".to_string()
        } else {
            format!("{} {:.0}/100", self.tier, self.average_score)
        }
    }

    pub fn platforms(&self) -> String {
        self.platforms
            .iter()
            .map(|platform| platform.short_name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn genres(&self) -> String {
        self.genres
            .iter()
            .map(|genre| genre.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn released_for(&self, platforms: &[String]) -> bool {
        self.platforms
            .iter()
            .any(|platform| platforms.contains(&platform.short_name))
    }

    pub fn released_today(&self) -> bool {
        let now = Local::now();
        let release = self.first_release_date;

        release.year() == now.year() && release.month() == now.month() && release.day() == now.day()
    }
}
