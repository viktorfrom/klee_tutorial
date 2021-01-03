#![allow(unused_imports, dead_code)]
extern crate env_logger;
extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod cli;
mod common;
mod srp_analysis;
mod tasks;
mod render_file;

use common::*;
use srp_analysis::*;
use tasks::*;

fn main() {
    let tasks = tasks();
    cli::cli(&tasks);

    // println!("tasks {:?}", &tasks);
    // println!("tot_util {}", tot_util(&tasks));

    // let (ip, tr) = pre_analysis(&tasks);
    // println!("ip: {:?}", ip);
    // println!("tr: {:?}", tr);

    // let approx = srp_analysis(&tasks, &ip, &tr, true);
    // println!("approx = {:#?}", approx);

    // let exact = srp_analysis(&tasks, &ip, &tr, false);
    // println!("exact = {:#?}", exact);
}
