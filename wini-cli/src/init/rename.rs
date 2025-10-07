use {
    super::{RepoSummary, err::InitError, git::first_commit},
    std::{fs, io, path::Path},
};

fn replace_in_file<P: AsRef<Path>>(
    file_path: P,
    target: &str,
    replacement: &str,
) -> io::Result<()> {
    let content = std::fs::read_to_string(&file_path)?;

    let new_content = content.replace(target, replacement);

    std::fs::write(&file_path, new_content)?;

    Ok(())
}

fn replace_in_directory<P: AsRef<Path>>(
    dir_path: P,
    target: &str,
    replacement: &str,
) -> io::Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Error in writing to a file aren't important
            let _ = replace_in_file(&path, target, replacement);
        } else if path.is_dir() {
            replace_in_directory(&path, target, replacement)?;
        }
    }

    Ok(())
}

pub fn rename_fields(repo_summary: &RepoSummary) -> Result<(), InitError> {
    for (from, to) in [
        ("HASH_TO_RESOLVE", repo_summary.last_commit_hash.as_str()),
        (
            "URL_TO_RESOLVE",
            repo_summary
                .remote_url
                .as_ref()
                .map_or("NONE", |e| e.as_str()),
        ),
        ("BRANCH_NAME_TO_RESOLVE", repo_summary.branch.as_str()),
        ("PROJECT_NAME_TO_RESOLVE", repo_summary.dir.as_str()),
    ] {
        replace_in_directory(&repo_summary.dir, from, to).map_err(InitError::IoError)?;
    }

    first_commit(&repo_summary.dir).map_err(InitError::OtherGitError)?;

    Ok(())
}
