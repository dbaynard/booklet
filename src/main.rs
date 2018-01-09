extern crate booklet;

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
