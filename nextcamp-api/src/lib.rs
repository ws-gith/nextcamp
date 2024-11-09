#![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]

pub mod authentication;
pub mod config;
pub mod lead;
pub mod letter;
pub mod smtp;
pub mod user;
pub mod www;

#[macro_use]
extern crate std_plus;
