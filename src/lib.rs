#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate lopdf;

extern crate num;
#[macro_use]
extern crate num_derive;

use lopdf::Document;
use lopdf::Object;
use lopdf::Dictionary;
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
    let po = pp.print_order().map(|original_page| match original_page {
        None => {
            in_pages.get(&1)
                .and_then(|&x| doc.get_object(x))
                .and_then(Object::as_dict)
                .map(Dictionary::clone)
                .and_then(|mut x| Dictionary::remove(&mut x, "Contents"))
                .map(|blank| doc.add_object(blank))
        },
        Some(p) => {
            in_pages.get(&p)
                .map(|x| *x)
        },
    });

    Ok(())
}

pub fn nonzeroError() -> io::Error {
    io::Error::new(InvalidInput, "Need nonzero document length")
}
