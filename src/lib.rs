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
            if let Some(blank) = in_pages.get(&1)
                .and_then(|&x| doc.get_object(x))
                .and_then(Object::as_dict)
                .map(Dictionary::clone)
                .and_then(|mut x| Dictionary::remove(&mut x, "Contents"))
            {
                let blank_id = doc.add_object(blank);
                blank_id
            } else {
                panic!("Should be a valid id");
            }
        },
        Some(p) => {
            if let Some(&page_id) = in_pages.get(&p)
            {
                page_id
            } else {
                panic!("Should be a valid id");
            }
        },
    });

    Ok(())
}

pub fn nonzeroError() -> io::Error {
    io::Error::new(InvalidInput, "Need nonzero document length")
}
