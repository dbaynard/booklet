//! Rearrange pages within a pdf

use std::io;
use std::path::Path;
use extra::error::*;

/// `lopdf` handles the pdf processing
use lopdf::Document;
use lopdf::{Object,ObjectId};
use lopdf::Dictionary;
use std::collections::BTreeMap;
use extra::lopdf::GetObjectMut;

/// The page ordering
use calculate::*;

/// Pages are retrieved as a map
type PagesInfo = BTreeMap<u32, ObjectId>;

/// Rearrange the pages
///
/// TODO rename to rearrange
///
/// - infile: If a path, use the file, otherwise use stdin
/// - outfile: If a path, use the file, otherwise use stdout
pub fn reorder<P>(
    infile: Option<P>,
    outfile: Option<P>
    ) -> io::Result<()>
    where P: AsRef<Path>
{
    /// Load the input pdf
    let mut doc = match infile {
        Some(file_name) => Document::load(file_name),
        None => Document::load_from(io::stdin()),
    }?;

    /// Calculate page numbers and properties
    let in_pages = doc.get_pages();
    let pp = NonZero::new(in_pages.len() as u32)
        .map(|x| PageProps::new(&x))
        .ok_or(nonzero_error())?;

    /// Rearrange the pages
    rewrite_pages(&mut doc, &pp, &in_pages)?;

    /// Write the output pdf
    match outfile {
        Some(file_name) => {
            doc.save(file_name)?;
            Ok(())
        },
        None => doc.save_to(&mut io::stdout()),
    }

}

/// Generate new pages, insert and rearrange the resulting pdf pages.
fn rewrite_pages(
    doc: &mut Document,
    pp: &PageProps,
    in_pages: &PagesInfo,
    ) -> io::Result<()>
{
    /// Return a vector, rather than an iterator, to allow subsequent mutation of the document.
    ///
    /// TODO figure out how to return an iterator without comprimising this.
    let new_pages = generate_pages(doc, &pp, &in_pages)
        .map(Object::Reference)
        .collect();

    let pages_dict = pages_location(doc)?;

    let new_count = pp.new_pages() as i64;

    set_pages_dict(pages_dict, new_count, new_pages);

    Ok(())
}

/// Actually mutate the element listing pages of the pdf.
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

/// Return a mutable reference to the element of the pdf corresponding to the page ordering.
///
/// TODO write using `Borrow` to be polymorphic in reference mutability?
fn pages_location<'a>(doc: &'a mut Document) -> io::Result<&'a mut Dictionary>
{
    /// TODO it should be possible to pass the ‘Pages’ reference to get_object_mut directly
    let pages = doc.catalog()             // Option<&Dictionary>
        .and_then(|cat| cat.get("Pages")) // Option<&Object>
        .and_then(Object::as_reference)   // Option<&ObjectId>
        .ok_or(invalid("Can't find Pages reference"))?;

    doc.get_object_mut(pages)          // Option<&Object>
        .and_then(Object::as_dict_mut) // Option<&mut Dictionary>
        .ok_or(invalid("Can't find Pages dictionary"))
}

/// Process the list of pages and add any required blanks
///
/// TODO now that I get what `move` does is it possible to remove it?
fn generate_pages<'a>(
    doc: &'a mut Document,
    pp: &'a PageProps,
    in_pages: &'a PagesInfo,
    ) -> impl Iterator<Item=ObjectId> + 'a
{
    let f = pp.print_order().filter_map(move |original_page| match original_page {
        None => {
            in_pages.get(&1)
                .and_then(|&x| doc.get_object(x))
                .and_then(Object::as_dict)
                .map(Dictionary::clone)
                .map(|mut x| {
                    x.remove("Contents");
                    x
                })
                .map(|blank| doc.add_object(blank))
        },
        Some(p) => {
            in_pages.get(&p)
                .map(|x| *x)
        },
    });

    Box::new(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    /// This test requires the file test.pdf to be present.
    fn test_reorder() {
        let test_out = reorder(
            Some("test.pdf"),
            Some("test-out.pdf"),
        );

        assert_eq!(true, test_out.is_ok())
    }
}
