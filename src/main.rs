mod app;
mod git;
mod image;

use app::App;
use chrono::{Datelike, Days, NaiveDate, Utc, Weekday};

const IMAGE_OFFSET: u64 = 30;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = std::env::args()
        .nth(1)
        .ok_or("Missing first argument REPO_FOLDER: gh-crab <REPO_FOLDER>")?;

    let year_start = NaiveDate::from_ymd_opt(Utc::now().year(), 1, 1).unwrap();
    let grid_origin = year_start.week(Weekday::Sun).first_day();
    let image_start = grid_origin
        .checked_add_days(Days::new(IMAGE_OFFSET * 7))
        .ok_or("Image offset too large")?;

    let app = App::new(repo_path)?;
    app.generate_commits(&image_start)?;

    println!("✓ Generated commits for image pattern");
    Ok(())
}
