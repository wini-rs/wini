#![allow(incomplete_features, reason = "This is the only way to make it work")]
#![feature(specialization)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(hash_map_macro)]
#![feature(impl_trait_in_fn_trait_return)]
pub mod components;
pub mod cron;
pub mod layouts;
pub mod pages;
pub mod server;
pub mod shared;
pub mod template;
pub mod utils;
