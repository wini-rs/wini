use {
    crate::{
        shared::wini::config::SERVER_CONFIG,
        utils::wini::file::{self, get_files_in_directory_per_extensions},
    },
    dotenvy::dotenv,
    env::EnvType,
    err::ExitWithMessageIfErr,
    std::{
        collections::{HashMap, HashSet},
        ffi::OsStr,
        str::FromStr,
        sync::LazyLock,
    },
};

type FileContent = String;
type FileName = String;

/// The list of all the public endpoints <=> all the files in `../public`
pub static PUBLIC_ENDPOINTS: LazyLock<HashSet<String>> =
    LazyLock::new(|| file::get_files_in_directory(SERVER_CONFIG.path().public_from_src()));

/// An HashMap of all the CSS files, with their content being the value
pub static CSS_FILES: LazyLock<HashMap<FileName, FileContent>> = LazyLock::new(|| {
    get_files_in_directory_per_extensions("src", &[OsStr::new("css")], false)
        .into_iter()
        .map(|file| {
            let content = std::fs::read_to_string(&file[1..])
                .exit_with_msg_if_err("File should always exist.");

            (file, content)
        })
        .collect()
});

/// An HashMap of all the JavaScript files, with their content being the value
pub static JS_FILES: LazyLock<HashMap<FileName, FileContent>> = LazyLock::new(|| {
    get_files_in_directory_per_extensions("src", &[OsStr::new("js")], false)
        .into_iter()
        .map(|file| {
            let content = std::fs::read_to_string(format!(".{file}"))
                .exit_with_msg_if_err("File should always exist.");

            (file, content)
        })
        .collect()
});

pub static ENV_TYPE: LazyLock<EnvType> = LazyLock::new(|| {
    dotenv().exit_with_msg_if_err("Couldn't load environment.");

    EnvType::from_str(
        std::env::var("ENV_TYPE")
            .exit_with_msg_if_err("Couldn't load environment variable `ENV_TYPE`")
            .as_str(),
    )
    .exit_with_msg_if_err("Invalid kind of environment")
});

pub static PORT: LazyLock<u16> = LazyLock::new(|| {
    dotenv().exit_with_msg_if_err("Couldn't load environment.");

    std::env::var("PORT")
        .exit_with_msg_if_err("Port not specified in the environment:")
        .parse::<u16>()
        .exit_with_msg_if_err("Port is not a valid `u16`")
});


pub mod cache;
pub mod config;
pub mod dependencies;
pub mod env;
pub mod err;
pub mod layer;
pub mod layout;
pub mod packages_files;
pub mod response;
// IFFEAT ssg
//// IFTARGET sdk
#[cfg(feature = "ssg")]
//// ENDIF
pub mod ssg;
// ENDIF
pub mod tsconfig;
