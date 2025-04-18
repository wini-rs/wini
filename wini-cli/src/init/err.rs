use {crate::init::SEP, std::fmt::Display};

#[derive(Debug)]
pub enum InitError {
    CouldntCloneRepo(String),
    PathExistsButIsNotGit(String),
    BranchDoesntExists(String),
    ManualExit,
    CloneNeedsAuthentification,
    BadCredentials,
    AlreadyExists(String),
    IoError(std::io::Error),
    InvalidPath(String),
    OtherGitError(git2::Error),
    JustError(i32),
}


impl Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{SEP}\n\x1b[31m◆\x1b[91;1m {}",
            match self {
                Self::CouldntCloneRepo(url) => format!("Couldn't clone repo with url: {url}"),
                Self::ManualExit => String::from("See you next time!"),
                Self::BadCredentials => String::from("Bad credentials"),
                Self::PathExistsButIsNotGit(path) =>
                    format!("The path `{path}` exists, but is not a git repository."),
                Self::BranchDoesntExists(branch) => format!("Branch {branch} doesn't exists"),
                Self::InvalidPath(path) => format!("File `{path}` doesn't exists."),
                Self::CloneNeedsAuthentification =>
                    "You need to authenticate to clone this repository.".to_string(),
                Self::OtherGitError(err) => format!("{:?}", err.message()),
                Self::IoError(err) => err.to_string(),
                Self::AlreadyExists(path) => format!("There is already a directory at {path:#?}"),
                Self::JustError(exit_code) =>
                    format!("A just command failed with exit code: {exit_code}"),
            }
        ))
    }
}
