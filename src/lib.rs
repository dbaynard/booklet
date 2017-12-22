#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate lopdf;

extern crate num;
#[macro_use]
extern crate num_derive;

use lopdf::Document;

#[derive(Debug, FromPrimitive)]
enum OnLeaf {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Debug)]
enum Half {
    Former,
    Latter,
}
#[derive(Debug)]
struct PageProps {
    leaves: u32,
    new_pages: u32,
    start_page: u32,
    blanks: OnLeaf,
}

fn mk_page_props(pages: u32) -> PageProps {
    use num::FromPrimitive;

    // round up for the sheets of paper used
    let leaves = get_leaves(&pages);

    // four pages per leaf
    let new_pages = leaves * 4;

    // The first page is the middle face, LHS
    let start_page = leaves * 2;

    let blanks = FromPrimitive::from_u32(pages % 4)
        .expect("This cannot fail");

    PageProps {
        leaves,
        new_pages,
        start_page,
        blanks,
    }
}

fn get_leaves(pages: &u32) -> u32 {
    (pages - 1) / 4 + 1
}
