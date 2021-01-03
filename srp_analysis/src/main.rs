#![allow(unused_imports, dead_code)]
extern crate env_logger;
extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cli;
mod common;
mod render_file;
mod srp_analysis;
mod tasks;

use common::*;
use tasks::*;

fn main() {
    let tasks = tasks();
    cli::cli(&tasks);
}
