#![no_std]
#![forbid(unsafe_code)]
#![deny(
    rust_2018_idioms,
    redundant_lifetimes,
    redundant_semicolons,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::allow_attributes
)]
#![allow(
    clippy::enum_glob_use,
    clippy::wildcard_imports,
    clippy::missing_errors_doc,
    clippy::doc_markdown
)]

mod config;
mod constants;
mod mode;
mod touchpad;
pub use config::*;
pub(crate) use constants::*;
pub use mode::*;
pub use touchpad::*;
