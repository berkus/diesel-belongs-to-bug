#![feature(custom_attribute)]
#![recursion_limit="256"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;

pub mod schema;
pub mod models;
