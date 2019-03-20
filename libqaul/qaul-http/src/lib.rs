//!

// #![feature(plugin)]
// #![plugin(rocket_codegen)]

// extern crate rocket;
// extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod models;
// mod rest;

pub fn run() {
    // Bootstrap REST API and launch it
    // rest::initialise().launch();
}
