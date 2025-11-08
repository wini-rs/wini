#![allow(incomplete_features, reason = "This is the only way to make it work")]
#![feature(specialization)]
#![feature(never_type)]
#![feature(impl_trait_in_fn_trait_return)]
mod shared;
pub mod template;
mod utils;

pub use {shared::wini::*, utils::wini::*};
pub mod cache {
    pub use {shared::wini::cache::*, utils::wini::cache::*};

    use super::*;
}
