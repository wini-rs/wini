use wini::init::git::clone;

#[test]
fn cloning_and_init_test() {
    assert_eq!(
        clone("https://github.com/wini-rs/wini-template").unwrap(),
        "wini-template"
    );
}

#[test]
fn bad_repository() {
    assert!(clone("https://github.com/wini-rs/doesnt-exists").is_err());
}
