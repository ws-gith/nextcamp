#![cfg_attr(debug_assertions, allow(unused_variables, dead_code, unused_imports))]
#[allow(unused_imports)]
#[macro_use]
extern crate std_plus;
#[macro_use]
extern crate tracing;

pub use std_plus::*;
pub mod analytics;
pub mod campaign;
pub mod contacts;
pub mod user;

/*
// TODO:  -->

Build user facade
Implement auth
Build auth facade
*/
