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

    let ps = NonZero::new(in_pages.len())
        .ok_or(io::Error::new(InvalidInput, "Need nonzero document length"))?;

    Ok(())
}
