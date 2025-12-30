use chrono::{Days, prelude::*};
use git2::{Repository, Signature, Time};
use std::path::Path;

const IMAGE_OFFSET: u64 = 30;
const IMAGE: [[i32; 16]; 7] = [
    [0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0],
    [4, 0, 3, 0, 0, 3, 3, 4, 4, 3, 3, 0, 0, 3, 0, 4],
    [1, 3, 2, 0, 3, 3, 0, 4, 4, 0, 3, 3, 0, 2, 3, 1],
    [0, 1, 2, 4, 3, 4, 4, 4, 4, 4, 4, 3, 4, 2, 1, 0],
    [0, 0, 1, 2, 3, 3, 3, 0, 0, 3, 3, 3, 2, 1, 0, 0],
    [0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
    [0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0],
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(first_day_of_year) = NaiveDate::from_ymd_opt(Utc::now().year(), 1, 1) else {
        panic!("The specified calendar day does not exist or the year is out of range.");
    };

    let first_contrib_week = first_day_of_year.week(Weekday::Sun);
    let contrib_start_date = first_contrib_week.first_day();
    let Some(image_start_date) = contrib_start_date.checked_add_days(Days::new(IMAGE_OFFSET * 7))
    else {
        panic!("checked_add_days() resulting date is out of range.");
    };

    let repo_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "art-repo".to_string());
    let repo = Repository::open(&repo_path)?;

    // Start fresh
    reset_branch(&repo)?;

    let width = IMAGE[0].len();
    let height = IMAGE.len();

    // Create commits based on the image
    for x in 0..width {
        for y in 0..height {
            let Some(commit_date) =
                image_start_date.checked_add_days(Days::new((x * 7 + y).try_into().unwrap()))
            else {
                panic!("checked_add_days() resulting date is out of range.");
            };

            let timestamp = commit_date
                .and_hms_opt(12, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp();

            // Make multiple commits based on strength value
            for _ in 0..match IMAGE[y][x] {
                1 => 1,
                2 => 7,
                3 => 14,
                4 => 20,
                _ => 0,
            } {
                commit(&repo, timestamp)?;
            }
        }
    }

    println!("✓ Generated commits for image pattern");
    Ok(())
}

fn commit(repo: &Repository, timestamp: i64) -> Result<(), git2::Error> {
    let workdir = repo.workdir().expect("Repository has no working directory");
    let art_file = workdir.join("art.txt");

    // Modify file in the repo
    std::fs::write(&art_file, format!("commit at {}", timestamp)).unwrap();

    // Stage it
    let mut index = repo.index()?;
    index.add_path(Path::new("art.txt"))?;
    index.write()?;

    // Get the tree
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Get parent commit (if exists)
    let parent_commit = repo.head().ok().and_then(|head| head.peel_to_commit().ok());
    let parents: Vec<_> = parent_commit.iter().collect();

    // Create commit with custom timestamp
    let time = Time::new(timestamp, 0);
    let sig = Signature::new("Samuel Braun", "sam@webry.com", &time)?;

    repo.commit(
        Some("HEAD"), // Update HEAD
        &sig,
        &sig,
        "•",
        &tree,
        &parents,
    )?;

    Ok(())
}

fn reset_branch(repo: &Repository) -> Result<(), Box<dyn std::error::Error>> {
    let workdir = repo.workdir().expect("Repository has no working directory");
    let art_file = workdir.join("art.txt");

    // Create initial commit
    std::fs::write(&art_file, "initial")?;

    let mut index = repo.index()?;
    index.add_path(Path::new("art.txt"))?;
    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = Signature::now("Samuel Braun", "sam@webry.com")?;

    // Create orphan commit
    let commit_id = repo.commit(None, &sig, &sig, "Initial commit", &tree, &[])?;

    // Force update main to point here
    repo.reference("refs/heads/main", commit_id, true, "Reset to orphan")?;
    repo.set_head("refs/heads/main")?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

    Ok(())
}
