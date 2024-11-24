use {
    crate::utils::wini::file::{self, get_files_in_directory_per_extensions},
    dotenvy::dotenv,
    env::EnvType,
    err::ExitWithMessageIfErr,
    std::{collections::HashMap, str::FromStr, sync::LazyLock},
};

/// The list of all the public endoints <=> all the files in `../public`
pub static PUBLIC_ENDPOINTS: LazyLock<Vec<String>> =
    LazyLock::new(|| file::get_files_in_directory("./public").unwrap_or_else(|_| Vec::new()));

/// An HashMap of all the CSS files, with their content being the value
pub static CSS_FILES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut hm = HashMap::new();

    for file in get_files_in_directory_per_extensions("src", &["css"]) {
        hm.insert(format!("/{file}"), std::fs::read_to_string(file).unwrap());
    }

    hm
});

/// An HashMap of all the JavaScript files, with their content being the value
pub static JS_FILES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut hm = HashMap::new();

    for file in get_files_in_directory_per_extensions("src", &["js"]) {
        hm.insert(
            format!("/{}", &file[..]),
            std::fs::read_to_string(file).unwrap(),
        );
    }

    hm
});

pub static ENV_TYPE: LazyLock<EnvType> = LazyLock::new(|| {
    dotenv().ok();
    let env_type = std::env::var("ENV_TYPE")
        .exit_with_msg_if_err("Couldn't load environment variable `ENV_TYPE`");
    EnvType::from_str(&env_type).exit_with_msg_if_err("Invalid kind of environment")
});

pub static PORT: LazyLock<u16> = LazyLock::new(|| {
    dotenv().ok();
    std::env::var("PORT")
        .exit_with_msg_if_err("Port not specified in the environment:")
        .parse::<u16>()
        .exit_with_msg_if_err("Port is not a valid `u16`")
});


pub mod cache;
pub mod components_files;
pub mod config;
pub mod dependencies;
pub mod env;
pub mod err;
pub mod packages_files;
pub mod tsconfig;
