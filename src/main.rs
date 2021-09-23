#[macro_use] extern crate prettytable;

mod opencritic;

use clap::{AppSettings, Clap};
use prettytable::Table;

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

async fn list_platforms() {
  let mut platforms = opencritic::api::get_platforms().await;

  platforms.sort_by(|a, b| a.name.cmp(&b.name));

  let mut table = Table::new();
  table.set_titles(row!["Name", "Short name"]);

  for platform in platforms {
    table.add_row(row![platform.name, platform.short_name]);
  }

  println!("Available platforms:");
  table.printstd();
}

async fn list_genres() {
  let mut genres = opencritic::api::get_genres().await;

  genres.sort_by(|a, b| a.name.cmp(&b.name));

  println!("Available genres:");

  for genre in genres {
    println!("- {}", genre.name);
  }
}

#[tokio::main]
async fn main() {
  let opts: Opts = Opts::parse();

  if opts.list_platforms {
    return list_platforms().await;
  }

  if opts.list_genres {
    return list_genres().await;
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

  let games = if opts.ignore_date {
    opencritic::api::get_recent_releases(&platforms, &genres).await
  } else {
    opencritic::api::get_todays_releases(&platforms, &genres).await
  };

  if games.is_empty() {
    println!(
      "🔴 No relevant games released {} 😢",
      if opts.ignore_date { "recently" } else { "today" }
    );
  } else {
    let time = if opts.ignore_date {
      "Recent"
    } else {
      "Today's"
    };
    println!("📀 {} releases:", time);

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
}
