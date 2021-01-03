use crate::common::*;
use crate::render_file::*;
use crate::srp_analysis::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "srp_analysis")]
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
    let mut analysis;

    if opt.approx {
        analysis = srp_analysis(&tasks, &ip, &tr, true);
    } else {
        analysis = srp_analysis(&tasks, &ip, &tr, false);
    }

    render_file(&tot_util, &analysis);
}
