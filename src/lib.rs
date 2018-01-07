#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate lopdf;

extern crate num;
#[macro_use]
extern crate num_derive;

use lopdf::Document;
use lopdf::{Object,ObjectId};
use lopdf::Dictionary;
use std::io;
use std::io::ErrorKind::*;

pub mod reorder;
pub use reorder::*;

type PagesInfo = std::collections::BTreeMap<u32, ObjectId>;

pub fn reorder(infile: &str) -> Result<(), io::Error> {
    let mut doc = Document::load(infile)?;

    let in_pages = doc.get_pages();
    let pp = NonZero::new(in_pages.len() as u32)
        .map(|x| PageProps::new(&x))
        .ok_or(nonzeroError())?;

    // TODO Need to get typechecker to enforce this is called
    rewrite_pages(&mut doc, &pp)?;

    let po = generate_pages(&mut doc, &pp, &in_pages)?;

    Ok(())
}

fn rewrite_pages(doc: &mut Document, pp: &PageProps) -> Result<(), io::Error> {
    let pages_loc = pages_location(&doc)
        .ok_or(invalid("Couldn’t find ‘Pages’ location"))?;

    let pages_dict = doc.get_object(pages_loc)
        .and_then(Object::as_dict)
        .ok_or(invalid("Couldn’t find ‘Pages’ dictionary"))?;

    pages_dict.get("Count")
        .iter().filter(
            |x| x.as_i64().is_some()
            ).next()
        .map(|_| Object::Integer(pp.new_pages() as i64))
        .ok_or(invalid("Couldn’t find ‘Count’ key"))?;

    pages_dict.get("Kids")
        .and_then(Object::as_array)
        .map(|v| v.iter()
            .filter_map(Object::as_reference)
            .zip((1..))
             )
        .ok_or(invalid("Couldn’t find ‘Kids’ key"))?;

    Ok(())
}

fn pages_location(doc: &Document) -> Option<ObjectId> {
    doc.catalog()
        .and_then(|cat| cat.get("Pages"))
        .and_then(Object::as_reference)
}

pub fn nonzeroError() -> io::Error {
    io::Error::new(InvalidInput, "Need nonzero document length")
}

fn invalid(err: &str) -> io::Error {
    io::Error::new(InvalidData, err)
}

fn generate_pages(doc: &mut Document, pp: &PageProps, in_pages: &PagesInfo) -> Result<(), io::Error> {
    pp.print_order().map(|original_page| match original_page {
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
