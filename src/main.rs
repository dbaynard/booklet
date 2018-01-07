extern crate booklet;

use booklet::*;
use std::io;

fn main() {
    match test_pages() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn test_pages() -> Result<(), io::Error> {
    let ps = NonZero::new(27).ok_or(nonzero_error())?;
    let pp = PageProps::new(&ps);
    let po = pp.print_order();

    println!("{:?}", po.collect::<Vec<_>>());
    reorder("test.pdf", "test-out.pdf")?;

    Ok(())
}
