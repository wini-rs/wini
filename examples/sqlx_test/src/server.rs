use {
    crate::{
        pages,
        shared::wini::PORT,
        template,
        utils::wini::{
            cache,
            handling_file::{self},
        },
    },
    axum::{middleware, routing::get, Router},
    dotenvy::dotenv,
    log::info,
    sqlx::postgres::PgPoolOptions,
    std::sync::LazyLock,
    tower_http::compression::CompressionLayer,
};
struct MyUser {
    name: String,
    age: Option<i32>,
}


pub async fn start() {
    // Support for compression
    let compression_layer = CompressionLayer::new();

    // let random_user = sqlx::query_as!(
    //     MyUser,
    //     r#"
    //     select name, age
    //     from users
    //     where id = 1;
    //     "#
    // )
    // .fetch_one(&*crate::POOL)
    // .await
    // .expect("An error occurred");


    // The main router of the application is defined here
    let app = Router::new()
        .route("/", get(pages::hello::render))
        .layer(middleware::from_fn(template::template))
        .layer(middleware::from_fn(cache::html_middleware))
        .route("/*.", get(handling_file::handle_file))
        .layer(compression_layer);


    // Start the server
    info!("Starting listening on port {}...", *PORT);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", *PORT))
        .await
        .expect("Couldn't start the TcpListener of the specified port.");

    info!("Starting the server...");
    axum::serve(listener, app)
        .await
        .expect("Couldn't start the server.");
}
