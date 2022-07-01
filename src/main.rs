/// The booklet library contains most of the code
extern crate booklet;
use booklet::*;

/// Argument parsing uses `clap` version 3 with its derive interface.
extern crate clap;
use clap::Parser;

/// # Main functions
fn main() {
    let args = Args::parse();

    match booklet(args) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn booklet(args: Args) -> lopdf::Result<()> {
    reorder(args.input, args.output)?;

    Ok(())
}

/// # Options
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file, if present (otherwise stdin)
    input: Option<String>,

    /// Output file, if present (otherwise stdout)
    output: Option<String>,
}
