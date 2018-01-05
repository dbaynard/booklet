extern crate booklet;

use booklet::{NonZero,PageProps};

fn main() {
    match test_pages() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn test_pages() -> Result<(), &'static str> {
    Ok(())
}
