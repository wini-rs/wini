use {
    super::err::InitError,
    crate::{
        init::{select, sep},
        utils::generate_random_string,
    },
    git2::{BranchType, Cred, CredentialType, IndexAddOption, Repository, Signature, Time},
    inquire::{Password, Text},
    std::{
        borrow::Cow,
        fmt::Write,
        iter::once,
        path::Path,
        time::{SystemTime, UNIX_EPOCH},
    },
};


/// Interactively authenticate a user
pub fn auth(_: &str, username: Option<&str>, _: CredentialType) -> Result<Cred, git2::Error> {
    println!("{}", InitError::CloneNeedsAuthentication);
    sep();

    let selection = select("Authenticate with", vec!["Key", "Password"])
        .map_err(|_| git2::Error::from_str("Manual Exit."))?;

    let username = if let Some(username) = username {
        Cow::Borrowed(username)
    } else {
        let username = Text::new("Username:")
            .prompt()
            .map_err(|_| git2::Error::from_str("Manual Exit."))?;

        Cow::Owned(username)
    };



    match selection {
        0 => {
            let mut path_to_key: Option<String> = None;

            while path_to_key.is_none() {
                let imaginary_path = Text::new("Path to key:")
                    .prompt()
                    .map_err(|_| git2::Error::from_str("Manual Exit."))?;

                if Path::new(&imaginary_path).exists() {
                    path_to_key = Some(imaginary_path);
                } else {
                    eprintln!("{}", InitError::InvalidPath(imaginary_path));
                    sep();
                }
            }

            Cred::ssh_key(
                &username,
                None,
                Path::new(&path_to_key.expect("Can't be None")),
                None,
            )
        },
        1 => {
            let mut password: Option<String> = None;

            while password.is_none() {
                let imaginary_path = Password::new("Password:")
                    .without_confirmation()
                    .prompt()
                    .map_err(|_| git2::Error::from_str("Manual Exit."))?;
                password = Some(imaginary_path);
            }

            Cred::userpass_plaintext(&username, &password.expect("Can't be None"))
        },
        _ => unreachable!(),
    }
}



/// Switch to a git branch, remove all other branch, and remove the remote repository.
pub fn use_branch(repo_path: &str, branch_name: &str) -> Result<String, git2::Error> {
    let repo = Repository::open(repo_path)?;

    let branch = repo.find_branch(&format!("origin/{branch_name}"), BranchType::Remote)?;

    // Reset the state to the current branch status
    let (object, _) = repo
        .revparse_ext(&format!("origin/{branch_name}"))
        .expect("Object not found");
    repo.reset(&object, git2::ResetType::Hard, None)?;

    // Create a local branch from the remote one
    let target_commit = branch.get().peel_to_commit()?;
    // Can error if already exists, in this case, do nothing.
    let _ = repo.branch(branch_name, &target_commit, false);

    let branch_ref = format!("refs/heads/{branch_name}");
    let branch_ref = repo.find_reference(&branch_ref)?;
    repo.set_head(branch_ref.name().unwrap_or_default())?;


    // Delete all other branches.
    let branch_to_keep = branch_name;
    let branches = repo.branches(Some(BranchType::Local))?;
    for branch in branches {
        let (mut branch, _) = branch?;
        let curr_branch_name = branch.name()?.unwrap_or_default();
        if curr_branch_name != branch_to_keep {
            branch.delete()?;
        }
    }

    // Delete remote and rename to main
    repo.remote_delete("origin")?;
    let mut branch = repo.find_branch(branch_to_keep, BranchType::Local)?;
    branch.rename("main", true)?;

    let last_commit_oid = target_commit.id();
    let mut last_commit_sha = String::with_capacity(40);
    for byte in last_commit_oid.as_bytes() {
        write!(&mut last_commit_sha, "{byte:02x}").expect("write");
    }


    Ok(last_commit_sha)
}


pub fn clone(url: &str) -> Result<String, InitError> {
    let clone_to = generate_random_string(64);

    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(auth);

    let mut opts = git2::FetchOptions::new();
    opts.remote_callbacks(callbacks);
    opts.download_tags(git2::AutotagOption::All);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(opts);


    match builder.clone(url, Path::new(&clone_to)) {
        Ok(_repo) => Ok(clone_to),
        Err(why) => {
            match why.code() {
                git2::ErrorCode::Auth => {
                    println!("{}", InitError::BadCredentials);

                    sep();

                    clone(url)
                },
                _ => Err(InitError::OtherGitError(why)),
            }
        },
    }
}


/// Makes the first commit that is unique to this repository.
/// There is content to commit because of the modification of Cargo.toml and wini.toml
pub fn first_commit(repo_path: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut index = repo.index()?;
    index.add_all(once("*"), IndexAddOption::DEFAULT, None)?;
    index.write()?;


    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("UNIX_EPOCH is always a valid date.")
        .as_secs();

    let author = Signature::new(
        "Wini",
        "wini",
        &Time::new(
            i64::try_from(now).expect("Current timestamp should be a valid i64"),
            0,
        ),
    )?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let parent_ids = repo.head()?.peel_to_commit()?;

    repo.commit(
        Some("HEAD"),
        &author,
        &author,
        "Creation of project",
        &tree,
        &[&parent_ids],
    )?;


    Ok(())
}
