extern crate booklet;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
use structopt::StructOpt;

use booklet::*;
use std::io;

fn main() {
    match booklet() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn booklet() -> io::Result<()> {
    Ok(())
}

#[derive(StructOpt)]
#[structopt(about="Rearrange pdf pages for booklet printing")]
struct Opt {
    /// Input file, if present (otherwise stdin)
    input: Option<String>,
    /// Output file, if present (otherwise stdout)
    output: Option<String>,
}
