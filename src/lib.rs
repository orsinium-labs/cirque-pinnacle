// https://static1.squarespace.com/static/53233e4be4b044fa7626c453/t/599de7856f4ca3c38aa74632/1503520647200/gt-an-090620_2-4_interfacingtopinnacle_i2c-spi_docver1-6.pdf
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
