
use std::path::PathBuf;
use structopt::StructOpt;
use crate::common::*;
use crate::srp_analysis::*;
use crate::render_file::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "v", long = "velocity", default_value = "42")]
    speed: f64,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    // /// Output file, stdout if not present
    // #[structopt(parse(from_os_str))]
    // output: Option<PathBuf>,

    // /// Where to write the output: to `stdout` or `file`
    // #[structopt(short)]
    // out_type: String,

    // /// File name: only required when `out-type` is set to `file`
    // #[structopt(name = "FILE", required_if("out-type", "file"))]
    // file_name: Option<String>,
}

pub fn cli(tasks: &Vec<Task>) {
    
    let (ip, tr) = pre_analysis(&tasks);
    let approx = srp_analysis(&tasks, &ip, &tr, true);
    let exact = srp_analysis(&tasks, &ip, &tr, false);

    let opt = Opt::from_args();
    // println!("{:?} {:#?} {:#?}", opt, approx, exact);
    println!("{:?}", opt);

    test();

}


