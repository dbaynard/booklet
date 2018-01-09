extern crate booklet;

use booklet::*;
use std::io;

fn main() {
    match test_pages() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn test_pages() -> io::Result<()> {
    reorder("test.pdf", "test-out.pdf")?;

    Ok(())
}
