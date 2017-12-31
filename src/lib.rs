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

#[derive(Debug, FromPrimitive, Clone, Copy)]
enum OnLeaf {
    Nil,
    One,
    Two,
    Three,
}

impl OnLeaf {
    fn new(x: u32) -> OnLeaf {
        use num::FromPrimitive;

        FromPrimitive::from_u32((4 - x % 4) % 4)
            .expect("This cannot fail")
    }
}

#[derive(Debug)]
enum Half {
    Former,
    Latter,
}
#[derive(Debug)]
pub struct PageProps {
    leaves: u32,
    new_pages: u32,
    start_page: u32,
    blanks: OnLeaf,
}

impl PageProps {
    pub fn new(pages: &NonZero<u32>) -> PageProps {
        // round up for the sheets of paper used
        let leaves = get_leaves(&pages);

        // four pages per leaf
        let new_pages = leaves * 4;

        // The first page is the middle face, LHS
        let start_page = leaves * 2;

        let blanks = OnLeaf::new(*pages.ex());

        PageProps {
            leaves,
            new_pages,
            start_page,
            blanks,
        }
    }

    pub fn next_page_no(&self, page: u32) -> u32 {
        let half = if page > self.leaves * 2 {
            Half::Latter
        } else {
            Half::Former
        };
        next_page_no(
            self.blanks,
            self.leaves,
            half,
            page,
            )
    }
}

#[derive(Debug, PartialEq)]
pub struct NonZero<T>(T);

impl<T: Unsigned> NonZero<T> {
    pub fn new(u: T) -> Option<NonZero<T>> {
        if u.is_zero() {
            None
        } else {
            Some(NonZero(u))
        }
    }

    fn ex(&self) -> &T {
        &self.0
    }
}

fn get_leaves(pages: &NonZero<u32>) -> u32 {
    (pages.ex() - 1) / 4 + 1
}

fn next_page_no(blanks: OnLeaf, leaves: u32, half: Half, page: u32) -> u32 {
    match (half, page % 2) {
        (Former,0) => 4 * leaves - page + 1,
        (Latter,0) => 4 * leaves - page - 1,
        (Former,1) => page - 1,
        (Latter,1) => page + 1,
        (_,_) => panic!("The impossible has happened.")
    }
}

#[cfg(test)]
mod test_get_leaves {
    use super::*;
    use quickcheck::TestResult;

    quickcheck! {
        fn prop_exact_leaves_correct(x: u32) -> TestResult {
            let y = (4 - x % 4) % 4;
            let x = match NonZero::new(x) {
                Some(r) => r,
                None => return TestResult::discard(),
            };
            let z = 4 * get_leaves(&x) - y;
            TestResult::from_bool(*x.ex() == z)
        }
    }
}
