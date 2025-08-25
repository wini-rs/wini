use {
    super::{RepoSummary, err::InitError, sep},
    crate::{
        init::{
            HEADER,
            OFFICIAL_REPOSITORY_BRANCHES,
            OFFICIAL_REPOSITORY_OPTIONS,
            RENDER_CONFIG,
            WINI_REPO,
            git::{clone, use_branch},
            input,
            rename::rename_fields,
            select,
        },
        utils::{copy_dir_all, generate_random_string},
    },
    git2::{BranchType, Repository},
    inquire::{Confirm, set_global_render_config},
    std::{fs, path::Path},
};



/// Ask the user how they want to create the project
pub fn ask() -> Result<(), InitError> {
    set_global_render_config(*RENDER_CONFIG);

    println!("{HEADER}");

    sep();

    let selection = select(
        "Create a project from",
        vec![
            "Official wini templates",
            "Remote git repository",
            "Local git repository",
        ],
    )?;

    sep();

    let repo_summary = match selection {
        0 => from_official_repository()?,
        1 => from_custom_remote_repository()?,
        2 => from_custom_local_repository()?,
        _ => unreachable!(),
    };

    rename_fields(&repo_summary)?;

    sep();

    println!(
        "\x1B[32m◆\x1B[0m Project created at `\x1B[32;1m./{}\x1b[0m`!",
        repo_summary.dir
    );

    Ok(())
}


/// Creates the repository project from one of the official template of wini
pub fn from_official_repository() -> Result<RepoSummary, InitError> {
    let handle_clone_official_repository = std::thread::spawn(|| clone(WINI_REPO));

    let result = (|| {
        let branch_index = select(
            "Which template should be used",
            OFFICIAL_REPOSITORY_OPTIONS.to_vec(),
        )?;
        let branch = OFFICIAL_REPOSITORY_BRANCHES[branch_index].to_owned();

        sep();

        let project_name = get_project_name()?;

        let path = Path::new(&project_name);
        if path.exists() && path.is_dir() {
            return Err(InitError::AlreadyExists(project_name));
        }

        Ok((project_name, branch))
    })();

    // We force the creation of the repository because if everything went right, we will need to
    // rename it.
    // Else, if there was an error, we will need to delete it. In both case, we want it to be
    // created.
    let current_repository_name = handle_clone_official_repository.join().unwrap()?;

    match result {
        Ok((project_name, branch)) => {
            // At this point the repository is created.
            // If, for some reason, it fails on the following closure, we will delete it.
            match (|| {
                fs::rename(&current_repository_name, &project_name).map_err(InitError::IoError)?;

                let last_commit_hash =
                    use_branch(&project_name, &branch).map_err(InitError::OtherGitError)?;

                Ok(RepoSummary {
                    dir: project_name,
                    branch,
                    last_commit_hash,
                    remote_url: Some(WINI_REPO.to_string()),
                })
            })() {
                Ok(summary) => Ok(summary),
                Err(err) => {
                    std::fs::remove_dir_all(current_repository_name).map_err(InitError::IoError)?;
                    Err(err)
                },
            }
        },
        Err(err) => {
            std::fs::remove_dir_all(current_repository_name).map_err(InitError::IoError)?;
            Err(err)
        },
    }
}



/// Ask information about the branch and the project name, and proceed to setup the repository
/// correctly.
pub fn handle_project_setup_for_custom(
    current_repository_name: &str,
    remote_url: Option<String>,
) -> Result<RepoSummary, InitError> {
    let branches = {
        let repo = Repository::open(current_repository_name).map_err(InitError::OtherGitError)?;
        let branches = repo
            .branches(Some(BranchType::Remote))
            .map_err(|_| InitError::OtherGitError(git2::Error::from_str("No branch found.")))?;

        branches
            .filter_map(|e| {
                e.ok().and_then(|(b, _)| {
                    b.name()
                        .ok()
                        .flatten()
                        .map(|name| name.replace("origin/", ""))
                })
            })
            .filter(|s| s != "HEAD")
            .collect::<Vec<String>>()
    };

    sep();

    let branch_index = select("Which branch should be used ?", branches.clone())?;
    let branch = &branches[branch_index];

    sep();

    let project_name = get_project_name()?;

    let path = Path::new(&project_name);
    if path.exists() && path.is_dir() {
        return Err(InitError::AlreadyExists(project_name));
    }

    fs::rename(current_repository_name, &project_name).map_err(InitError::IoError)?;

    let last_commit_hash = use_branch(&project_name, branch).map_err(InitError::OtherGitError)?;

    Ok(RepoSummary {
        dir: project_name,
        branch: branch.to_owned(),
        last_commit_hash,
        remote_url,
    })
}


/// Creates the repository project from a remote repository
pub fn from_custom_remote_repository() -> Result<RepoSummary, InitError> {
    let remote_url = input("Remote repository URL:")?;

    let current_repository_name = match clone(&remote_url) {
        Ok(n) => n,
        Err(InitError::OtherGitError(git_error)) => {
            if git_error.code() == git2::ErrorCode::NotFound ||
                git_error.class() == git2::ErrorClass::Http
            {
                eprintln!("{}", InitError::CouldntCloneRepo(remote_url));
                sep();

                return from_custom_remote_repository();
            } else {
                return Err(InitError::OtherGitError(git_error));
            }
        },
        Err(fail) => return Err(fail),
    };

    match handle_project_setup_for_custom(&current_repository_name, Some(remote_url)) {
        Ok(sum) => Ok(sum),
        Err(err) => {
            std::fs::remove_dir_all(current_repository_name).map_err(InitError::IoError)?;
            Err(err)
        },
    }
}



/// Creates the repository project from a local repository (probably not a good thing)
pub fn from_custom_local_repository() -> Result<RepoSummary, InitError> {
    // The repository path to copy from
    let repository_path = {
        let mut repository_path: Option<String> = None;

        while repository_path.is_none() {
            let input_repository_path = input("Local repository path:")?;

            let repository_path_struct = Path::new(&input_repository_path);

            if repository_path_struct.exists() {
                let path_of_git_dir_string = format!("{input_repository_path}/.git");
                let path_of_git_dir = Path::new(&path_of_git_dir_string);

                if path_of_git_dir.exists() && path_of_git_dir.is_dir() {
                    repository_path = Some(input_repository_path);
                } else {
                    eprintln!(
                        "{}",
                        InitError::PathExistsButIsNotGit(input_repository_path)
                    );
                }
            } else {
                eprintln!("{}", InitError::InvalidPath(input_repository_path));
            }
        }

        repository_path.expect("Can't be None.")
    };

    let current_repository_name = generate_random_string(64);
    copy_dir_all(repository_path, &current_repository_name).map_err(InitError::IoError)?;


    match handle_project_setup_for_custom(&current_repository_name, None) {
        Ok(sum) => Ok(sum),
        Err(err) => {
            std::fs::remove_dir_all(current_repository_name).map_err(InitError::IoError)?;
            Err(err)
        },
    }
}

fn get_project_name() -> Result<String, InitError> {
    let mut project_name = input("Project name:")?;

    if project_name.is_empty() {
        println!("{}", InitError::EmtpyProjectName);
        sep();
        return get_project_name();
    }

    let illegal_chars = project_name
        .chars()
        .filter(|c| !c.is_ascii_alphanumeric() && *c != '_')
        .collect::<Vec<_>>();

    if !illegal_chars.is_empty() {
        sep();
    }

    for char in illegal_chars {
        if project_name.contains(char) {
            let is_ok_for_renaming = Confirm::new(&format!(
                "Project name can't have a '{}' in it. Rename it to: \"{}\"",
                char,
                project_name.replace(char, "_")
            ))
            .with_default(true)
            .prompt()
            .unwrap_or(false);

            if is_ok_for_renaming {
                project_name = project_name.replace(char, "_");
            } else {
                return Err(InitError::ManualExit);
            }
        }
    }

    Ok(project_name)
}
