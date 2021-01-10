use crate::common::*;
use crate::render_file::*;
use crate::srp_analysis::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "srp_analysis",
    about = "
execute examples:
cargo run -- 
cargo run -- -a
cargo run -- --help
"
)]
struct Opt {
    #[structopt(short, long, default_value = "srp_analysis")]
    filename: String,

    #[structopt(short, long)]
    approx: bool,
}

pub fn cli(tasks: &Vec<Task>) {
    let opt = Opt::from_args();
    let (ip, tr) = pre_analysis(&tasks);
    let tot_util = tot_util(&tasks);
    let analysis;

    if opt.approx {
        analysis = srp_analysis(&tasks, &ip, &tr, true);
    } else {
        analysis = srp_analysis(&tasks, &ip, &tr, false);
    }

    render_file(&tot_util, &analysis);
    open_report();
}

fn open_report() {
    match open::that("target/srp_analysis.html") {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Opening file in browser");
            } else {
                if let Some(code) = exit_status.code() {
                    println!("Command returned non-zero exit status {}!", code);
                } else {
                    println!("Command returned with unknown exit status!");
                }
            }
        }
        Err(why) => println!("Failure to execute command: {}", why),
    }
}
