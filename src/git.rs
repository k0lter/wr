use std::{env, path::Path};

use anyhow::{anyhow, Error};
use git2::{Config, Cred, Remote, RemoteCallbacks, Repository};

use crate::{DEVELOP_BRANCH, MASTER_BRANCH};

pub fn ref_by_branch(branch: &str) -> String {
    format!("refs/heads/{}:refs/heads/{}", branch, branch)
}

pub fn ref_by_tag(tag: &str) -> String {
    format!("refs/tags/{}:refs/tags/{}", tag, tag)
}

/// Fetch credentials from the ssh-agent
pub fn create_remote_callback() -> Result<RemoteCallbacks<'static>, Error> {
    let mut cb = RemoteCallbacks::new();
    cb.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap())
    });

    Ok(cb)
}

pub fn get_gitflow_branch_name(branch: &str) -> String {
    let current_dir = env::current_dir().unwrap();
    let path = format!("{}/.git/config", current_dir.display());
    let config = Config::open(Path::new(&path)).unwrap();

    let config_path = format!("gitflow.branch.{}", &branch);
    config.get_string(&config_path).unwrap()
}

pub fn get_repository() -> Result<Repository, Error> {
    let current_dir = env::current_dir().unwrap();
    let repository = match Repository::open(current_dir) {
        Ok(repo) => repo,
        Err(_) => return Err(anyhow!("Please launch wr in a git repository.")),
    };

    Ok(repository)
}

pub fn get_remote(repository: &Repository) -> Result<Remote, Error> {
    let remote = repository.find_remote("origin")?;

    Ok(remote)
}

pub fn get_gitflow_branches_refs() -> Vec<String> {
    let branches = vec![MASTER_BRANCH.to_string(), DEVELOP_BRANCH.to_string()];
    let branches_refs: Vec<String> = branches.iter().map(|a| ref_by_branch(a)).collect();
    branches_refs
}
