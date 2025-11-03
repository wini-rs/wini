use {
    std::sync::LazyLock,
    PROJECT_NAME_TO_RESOLVE::{
        cron,
        server,
        shared::wini::{
            config::SERVER_CONFIG,
            dependencies::SCRIPTS_DEPENDENCIES,
            packages_files::PACKAGES_FILES,
            tsconfig::TSCONFIG_PATHS,
            CSS_FILES,
            ENV_TYPE,
            JS_FILES,
            PUBLIC_ENDPOINTS,
        },
    },
};

#[tokio::main]
async fn main() {
    // Init color syntaxing
    colog::init();

    // Lock all the environment data that we will use in our application so it's not 'uninit'
    LazyLock::force(&SERVER_CONFIG);
    LazyLock::force(&ENV_TYPE);
    LazyLock::force(&CSS_FILES);
    LazyLock::force(&JS_FILES);
    LazyLock::force(&PACKAGES_FILES);
    LazyLock::force(&TSCONFIG_PATHS);
    LazyLock::force(&PUBLIC_ENDPOINTS);
    LazyLock::force(&SCRIPTS_DEPENDENCIES);

    // Verify that all the kind of data returned by the server (html, css, js, etc.) have their
    // cache rules being correctly setup
    SERVER_CONFIG.cache().verify_all_attributes();

    cron::launch_crons().await;
    server::start().await;
}
