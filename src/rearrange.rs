use std::io;
use std::path::Path;
use std::collections::BTreeMap;

use lopdf::Document;
use lopdf::{Object,ObjectId};
use lopdf::Dictionary;

use reorder::*;

use extra::lopdf::GetObjectMut;
use extra::error::*;

type PagesInfo = BTreeMap<u32, ObjectId>;

pub fn reorder<P>(
    infile: Option<P>,
    outfile: Option<P>
    ) -> io::Result<()>
    where P: AsRef<Path>
{
    let mut doc = match infile {
        Some(file_name) => Document::load(file_name),
        None => Document::load_from(io::stdin()),
    }?;

    let in_pages = doc.get_pages();
    let pp = NonZero::new(in_pages.len() as u32)
        .map(|x| PageProps::new(&x))
        .ok_or(nonzero_error())?;

    rewrite_pages(&mut doc, &pp, &in_pages)?;

    match outfile {
        Some(file_name) => {
            doc.save(file_name)?;
            Ok(())
        },
        None => doc.save_to(&mut io::stdout()),
    }

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
