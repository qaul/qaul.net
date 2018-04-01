//!

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
}
