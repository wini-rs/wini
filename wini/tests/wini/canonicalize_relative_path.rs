use PROJECT_NAME_TO_RESOLVE::shared::wini::dependencies::normalize_relative_path;

#[test]
fn in_out_in() {
    let path = std::path::Path::new("src/../src");
    let normalized_path = normalize_relative_path(path);

    assert_eq!("src", normalized_path.display().to_string().as_str());
}

#[test]
fn relative_current() {
    let path = std::path::Path::new("./src");
    let normalized_path = normalize_relative_path(path);

    assert_eq!("src", normalized_path.display().to_string().as_str());
}

#[test]
fn in_out_many_times() {
    let path = std::path::Path::new(
        "src/../src/../src/../src/../src/../src/./../src/./../src/../src/../src/../src/../src",
    );
    let normalized_path = normalize_relative_path(path);

    assert_eq!("src", normalized_path.display().to_string().as_str());
}

#[test]
fn starting_with_dotdot() {
    let path = std::path::Path::new("../src");
    let normalized_path = normalize_relative_path(path);

    assert_eq!("../src", normalized_path.display().to_string().as_str());
}
