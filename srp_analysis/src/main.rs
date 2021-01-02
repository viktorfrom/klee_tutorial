mod common;
mod srp_analysis;
mod cli;
mod tasks;

use common::*;
use srp_analysis::*;
use tasks::*;

fn main() {
    let tasks = tasks();
    // cli::cli();    

    println!("tasks {:?}", &tasks);
    println!("tot_util {}", tot_util(&tasks));

    let (ip, tr) = pre_analysis(&tasks);
    println!("ip: {:?}", ip);
    println!("tr: {:?}", tr);

    let approx = srp_analysis(&tasks, &ip, &tr, true);
    println!("approx = {:#?}", approx);

    let exact = srp_analysis(&tasks, &ip, &tr, false);
    println!("exact = {:#?}", exact);
}
