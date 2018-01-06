#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate lopdf;

extern crate num;
#[macro_use]
extern crate num_derive;

use lopdf::Document;
use std::io;
use std::io::ErrorKind::*;

pub mod reorder;
pub use reorder::*;

pub fn reorder(infile: &str) -> Result<(), io::Error> {
    let mut doc = Document::load(infile)?;
    let mut in_pages = doc.get_pages();

    let ps = NonZero::new(in_pages.len() as u32)
        .ok_or(nonzeroError())?;

    let pp = PageProps::new(&ps);
    let po = pp.print_order();

    po.for_each(|r| {
        match r {
            None => {
                println!("Blank");
            },
            Some(r) => {
                if let Some(x) = in_pages.get_mut(&r) {
                    println!("{:?}", doc.get_object(*x));
                }
            },
        }
    });

    Ok(())
}

pub fn nonzeroError() -> io::Error {
    io::Error::new(InvalidInput, "Need nonzero document length")
}
