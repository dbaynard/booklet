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
    doc: &mut Document,
    pp: &PageProps,
    in_pages: &PagesInfo,
    ) -> io::Result<()>
{
    let new_pages = generate_pages(doc, &pp, &in_pages)
        .map(Object::Reference)
        .collect();

    let pages_dict = pages_location(doc)?;

    let new_count = pp.new_pages() as i64;

    set_pages_dict(pages_dict, new_count, new_pages);

    Ok(())
}

fn set_pages_dict(
    pages_dict: &mut Dictionary,
    new_count: i64,
    new_pages: Vec<Object>,
    )
{
    pages_dict.set(
        "Kids",
         Object::Array(new_pages),
    );

    pages_dict.set(
        "Count",
        Object::Integer(new_count),
    );
}

fn pages_location<'a>(doc: &'a mut Document) -> io::Result<&'a mut Dictionary>
{
    let pages = doc.catalog()             // Option<&Dictionary>
        .and_then(|cat| cat.get("Pages")) // Option<&Object>
        .and_then(Object::as_reference)   // Option<&ObjectId>
        .ok_or(invalid("Can't find Pages reference"))?;

    doc.get_object_mut(pages)          // Option<&Object>
        .and_then(Object::as_dict_mut) // Option<&mut Dictionary>
        .ok_or(invalid("Can't find Pages dictionary"))
}

trait GetObjectMut {
    fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object>;
}

impl GetObjectMut for Document {
    /// Get mutable object by object id, will recursively dereference a referenced object.
    fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        let is_ref;

        if let Some(object) = self.objects.get(&id) {
            is_ref = object.as_reference();
        } else {
            return None
        }

        if let Some(id) = is_ref {
            return self.get_object_mut(id);
        } else {
            return self.objects.get_mut(&id);
        }
    }
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
