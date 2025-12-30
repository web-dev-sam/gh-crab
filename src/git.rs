use git2::{Repository, Signature, Time};
use std::path::{Path, PathBuf};

pub fn commit(
    repo: &Repository,
    committing_file: &Path,
    timestamp: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(committing_file, "Ferris likes you! 🦀")?;

    let mut index = repo.index()?;
    index.add_path(Path::new("art.txt"))?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let parent_commit = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
    let parents: Vec<&git2::Commit> = parent_commit.iter().collect();

    let time = Time::new(timestamp, 0);
    let sig = git_signature(repo, Some(&time))?;

    repo.commit(Some("HEAD"), &sig, &sig, "•", &tree, &parents)?;
    Ok(())
}

pub fn reset_branch(
    repo: &Repository,
    committing_file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write(committing_file, "initial")?;

    let mut index = repo.index()?;
    index.add_path(Path::new("art.txt"))?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = git_signature(repo, None)?;

    let commit_id = repo.commit(None, &sig, &sig, "Initial commit", &tree, &[])?;

    repo.reference("refs/heads/main", commit_id, true, "Reset to orphan")?;
    repo.set_head("refs/heads/main")?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

    Ok(())
}

fn git_signature(
    repo: &Repository,
    time: Option<&Time>,
) -> Result<Signature<'static>, Box<dyn std::error::Error>> {
    let config = repo.config()?;
    let name = config.get_string("user.name")?;
    let email = config.get_string("user.email")?;

    match time {
        Some(t) => Ok(Signature::new(&name, &email, t)?),
        None => Ok(Signature::now(&name, &email)?),
    }
}

pub fn committing_file(repo: &Repository) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let workdir = repo
        .workdir()
        .ok_or("Repository has no working directory")?;
    Ok(workdir.join("art.txt"))
}
