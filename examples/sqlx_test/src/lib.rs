#![feature(impl_trait_in_fn_trait_return)]

use {
    dotenvy::dotenv,
    sqlx::postgres::PgPoolOptions,
    std::{sync::LazyLock, time::Duration},
};
pub mod pages;
pub mod server;
pub mod shared;
pub mod template;
pub mod utils;

pub static POOL: LazyLock<sqlx::PgPool> = LazyLock::new(|| {
    dotenv().ok().expect("Couldn't load env");

    let temp_runtime = tokio::runtime::Runtime::new().unwrap();

    temp_runtime.block_on(async {
        PgPoolOptions::new()
            .max_connections(8)
            .acquire_timeout(Duration::from_secs(1))
            .idle_timeout(Duration::from_secs(1))
            .connect(&std::env::var("DATABASE_URL").expect("No DATABASE_URL"))
            .await
            .expect("Couldn't connect to the database")
    })
});

#[ctor::ctor]
fn init_pool() {
    println!("tets!");
    LazyLock::force(&POOL);
}
