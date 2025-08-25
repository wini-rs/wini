use wini::init::git::clone;

#[test]
fn clone_returns_repo_dir() {
    let dir = clone("https://github.com/wini-rs/wini-template").unwrap();
    let path = std::path::Path::new(&dir);

    assert!(
        path.exists() && path.is_dir(),
        "clone() must create a directory"
    );
    assert!(
        path.join(".git").exists(),
        "cloned directory must contain a .git folder"
    );

    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn bad_repository() {
    assert!(
        clone("https://github.com/wini-rs/doesnt-exists").is_err(),
        "expected cloning a non-existent repo to error"
    );
}
