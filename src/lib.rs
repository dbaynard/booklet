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
use std::fs::File;

pub mod reorder;
pub use reorder::*;

type PagesInfo = std::collections::BTreeMap<u32, ObjectId>;

pub fn reorder<P>(infile: P, outfile: P) -> io::Result<File>
    where P: AsRef<std::path::Path>
{
    let mut doc = Document::load(infile)?;

    let in_pages = doc.get_pages();
    let pp = NonZero::new(in_pages.len() as u32)
        .map(|x| PageProps::new(&x))
        .ok_or(nonzero_error())?;

    rewrite_pages(&mut doc, &pp, &in_pages)?;

    doc.save(outfile)
}

fn rewrite_pages(
    mut doc: &mut Document,
    pp: &PageProps,
    in_pages: &PagesInfo,
    ) -> io::Result<()>
{
    let new_pages = Object::Array(
        generate_pages(&mut doc, &pp, &in_pages)
        .map(Object::Reference)
        .collect()
        );

    let mut pages_dict = pages_location(&doc)
        .ok_or(invalid("Couldn’t find ‘Pages’ dictionary"))?;

    pages_dict.set(
        "Count",
        Object::Integer(pp.new_pages() as i64),
    );

    pages_dict.set(
        "Kids",
        new_pages,
    );

    Ok(())
}

fn pages_location(doc: &Document) -> Option<&Dictionary> {
    doc.catalog()
        .and_then(|cat| cat.get("Pages"))
        .and_then(Object::as_reference)
        .and_then(|x| doc.get_object(x))
        .and_then(Object::as_dict)
}

pub fn nonzero_error() -> io::Error {
    io::Error::new(InvalidInput, "Need nonzero document length")
}

fn invalid(err: &str) -> io::Error {
    io::Error::new(InvalidData, err)
}

fn generate_pages<'a>(
    doc: &'a mut Document,
    pp: &'a PageProps,
    in_pages: &'a PagesInfo,
    ) -> Box<Iterator<Item=ObjectId> + 'a>
{
    let f = pp.print_order().filter_map(move |original_page| match original_page {
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

    Box::new(f)
}
