use chrono::NaiveDate;
use git2::Repository;
use std::path::{Path, PathBuf};

use crate::{
    git::{commit, committing_file, reset_branch},
    image::commit_timestamps,
};

pub struct App {
    repo: Repository,
    committing_file: PathBuf,

    /// Pixel art representation where each number indicates commit intensity:
    /// 0 = no commits, 1 = 1 commit, 2 = 7 commits, 3 = 14 commits, 4 = 20 commits
    image: [[i32; 16]; 7],
}

impl App {
    pub fn new(repo_path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let repo = Repository::open(repo_path)?;
        let committing_file = committing_file(&repo)?;

        reset_branch(&repo, &committing_file)?;

        // Ferris the crab and your emotional support plushie:
        //
        //       ████████
        //  █ ▓ ███████████ ▓ █
        //   ▒████  ████  ███▒
        //   ▒█████████████▒
        //    ▒████  ████▒
        //     ▒████████▒
        //      ▒ ▒  ▒ ▒
        //
        Ok(Self {
            repo,
            committing_file,
            image: [
                [0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0],
                [4, 0, 3, 0, 0, 3, 3, 4, 4, 3, 3, 0, 0, 3, 0, 4],
                [1, 3, 2, 0, 3, 3, 0, 4, 4, 0, 3, 3, 0, 2, 3, 1],
                [0, 1, 2, 4, 3, 4, 4, 4, 4, 4, 4, 3, 4, 2, 1, 0],
                [0, 0, 1, 2, 3, 3, 3, 0, 0, 3, 3, 3, 2, 1, 0, 0],
                [0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0],
                [0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0],
            ],
        })
    }

    pub fn generate_commits(
        &self,
        image_start: &NaiveDate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for timestamp in commit_timestamps(&self.image, image_start) {
            commit(&self.repo, &self.committing_file, timestamp)?;
        }
        Ok(())
    }
}
