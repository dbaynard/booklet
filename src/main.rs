/// The booklet library contains most of the code
extern crate booklet;
use booklet::*;

/// Argument parsing uses `structopt`
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

use std::io;

/// # Main functions
fn main() {
    let opt = Opt::from_args();

    match booklet(opt) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn booklet(opt: Opt) -> io::Result<()> {
    reorder(opt.input, opt.output)?;

    Ok(())
}

/// # Options
#[derive(StructOpt)]
struct Opt {
    /// Input file, if present (otherwise stdin)
    input: Option<String>,
    /// Output file, if present (otherwise stdout)
    output: Option<String>,
}
