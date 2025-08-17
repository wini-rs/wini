use wini::init::git::clone_and_init;

#[test]
fn cloning_and_init_test() {
    assert_eq!(
        clone_and_init("https://github.com/wini-rs/wini-template").unwrap(),
        "wini-template"
    );
}

#[test]
fn bad_repository() {
    assert!(clone_and_init("https://github.com/wini-rs/doesnt-exists").is_err());
}
