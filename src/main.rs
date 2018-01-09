extern crate booklet;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

use booklet::*;
use std::io;

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

#[derive(StructOpt)]
struct Opt {
    /// Input file, if present (otherwise stdin)
    input: Option<String>,
    /// Output file, if present (otherwise stdout)
    output: Option<String>,
}
