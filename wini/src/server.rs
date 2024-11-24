use {
    crate::{
        layout::header,
        pages,
        shared::wini::PORT,
        template,
        utils::wini::{
            cache,
            handling_file::{self},
        },
    },
    axum::{middleware, routing::get, Router},
    log::info,
    tower_http::compression::CompressionLayer,
};


pub async fn start() {
    // Support for compression
    let comression_layer = CompressionLayer::new();


    // The main router of the application is defined here
    let app = Router::new()
        .route("/", get(pages::hello::render))
        .layer(middleware::from_fn(header::render))
        .layer(middleware::from_fn(template::template))
        .layer(middleware::from_fn(cache::html_middleware))
        .route("/*.", get(handling_file::handle_file))
        .layer(comression_layer);


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
