#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate lopdf;

extern crate num;
#[macro_use]
extern crate num_derive;

use num::Unsigned;
use num::Zero;
use lopdf::Document;

#[derive(Debug, FromPrimitive)]
enum OnLeaf {
    Nil,
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

fn mk_page_props(pages: &NonZero<u32>) -> PageProps {
    use num::FromPrimitive;

    // round up for the sheets of paper used
    let leaves = get_leaves(&pages);

    // four pages per leaf
    let new_pages = leaves * 4;

    // The first page is the middle face, LHS
    let start_page = leaves * 2;

    let blanks = FromPrimitive::from_u32(pages.ex() % 4)
        .expect("This cannot fail");

    PageProps {
        leaves,
        new_pages,
        start_page,
        blanks,
    }
}

#[derive(Debug, PartialEq)]
enum NonZero<T> {
    NonZero(T),
}

fn mk_non_zero<T: Unsigned>(u: T) -> Option<NonZero<T>> {
    if u.is_zero() {
        None
    } else {
        Some(NonZero::NonZero(u))
    }
}

impl<T> NonZero<T> {
    fn ex(&self) -> &T {
        use NonZero::NonZero;
        match self {
            &NonZero(ref t) => t,
        }
    }
}

fn get_leaves(pages: &NonZero<u32>) -> u32 {
    (pages.ex() - 1) / 4 + 1
}

#[cfg(test)]
mod test_get_leaves {
    use super::*;

    quickcheck! {
        fn prop_exact_leaves_correct(x: u32) -> bool {
            use NonZero::NonZero;
            let y = match mk_non_zero(x / 4) {
                Some(r) => r,
                None => NonZero(1),
            };
            let z = get_leaves(&y);
            println!("{:?}, {:?}", y.ex(), z);
            *y.ex() == z
        }
    }
}
