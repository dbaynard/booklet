//! Calculate the order of pages for printing a pdf as a booklet
//!
//! A booklet prints 4 A4 pages in A5, two on each side (2-up).
//!
//! TODO Reverse the calculation (also enables property tests).

/// How many pages are blank on the final leaf?
#[derive(Debug, FromPrimitive, ToPrimitive, Clone, Copy, PartialEq)]
enum OnLeaf {
    Nil,
    One,
    Two,
    Three,
}

impl OnLeaf {
    /// TODO Link to quickcheck test
    ///
    /// Note: the modulo operations are safe for unsigned ints
    fn new(x: u32) -> OnLeaf {
        use num::FromPrimitive;

        FromPrimitive::from_u32((4 - x % 4) % 4)
            .expect("This cannot fail")
    }
}

/// Into which half of the document does a given page fall?
///
/// The half is important for directing the subsequent page.
#[derive(Debug)]
enum Half {
    Former,
    Latter,
}

/// The set of page properties
#[derive(Debug)]
pub struct PageProps {
    /// How many sheets of paper.
    leaves: u32,
    /// How many pages of content.
    pages: u32,
    /// How many pages total, including blanks.
    new_pages: u32,
    /// Which is the first page for the rearranged output.
    start_page: u32,
    /// How many blank pages added at the end.
    ///
    /// TODO This is currently unused.
    blanks: OnLeaf,
}

impl PageProps {
    /// This is only valid for a positive integer so the input is restricted to that.
    pub fn new(pgs: &NonZero<u32>) -> PageProps {
        let pages = *pgs.ex();

        // round up for the sheets of paper used
        let leaves = get_leaves(&pgs);

        // four pages per leaf
        let new_pages = leaves * 4;

        // The first page is the middle face, LHS
        let start_page = leaves * 2;

        let blanks = OnLeaf::new(pages);

        PageProps {
            leaves,
            pages,
            new_pages,
            start_page,
            blanks,
        }
    }

    /// Given an original page number, what original page is next?
    pub fn next_page_no(&self, page: u32) -> u32 {
        let half = if page > self.start_page {
            Half::Latter
        } else {
            Half::Former
        };
        next_page_no(
            self.new_pages,
            half,
            page,
            )
    }

    /// Produce an iterator for the new order of original page numbers
    pub fn print_order<'a>(&'a self) -> impl Iterator<Item = Option<u32>> + 'a
    {
        PageList::new(self).print_order()
    }

    /// Rather than making the `new_pages` field public, provide the same value with a function.
    pub fn new_pages(&self) -> u32 {
        self.new_pages
    }
}

/// TODO Enforce difference between pages and leaves with newtypes
/// TODO Link to quickcheck
fn get_leaves(pages: &NonZero<u32>) -> u32 {
    (pages.ex() - 1) / 4 + 1
}

/// This (private) function implements the logic of the corresponding `next_page_no` method.
///
/// # Errors
///
/// The exhaustivity of the pattern matches cannot be checked by the compiler, hence the panic.
fn next_page_no(pages: u32, half: Half, page: u32) -> u32 {
    use self::Half::*;

    match (half, page % 2) {
        (Former,0) => pages - page + 1,
        (Latter,0) => pages - page + 1,
        (Former,1) => page - 1,
        (Latter,1) => page + 1,
        (_,_) => panic!("The impossible has happened.")
    }
}

/// This type is required to iterate through the page numbers
#[derive(Debug)]
pub struct PageList<'a>(Option<u32>, &'a PageProps);

impl<'a> PageList<'a> {
    /// The start state has no page number, meaning the iterator returns the first page with the
    /// first iteration.
    pub fn new(pp: &'a PageProps) -> PageList<'a> {
        PageList(None, pp)
    }

    /// Any numbers outside the original page number list correspond to blank pages, represented
    /// here as `None`.
    pub fn print_order(self) -> impl Iterator<Item = Option<u32>> + 'a
    {
        let p = &self.1.pages;
        let f = self.map(move |x| {
            if x > *p {
                None
            } else {
                Some(x)
            }
        });

        Box::new(f)
    }
}

impl<'a> Iterator for PageList<'a> {
    type Item = u32;

    /// Set and return the same state
    fn next(&mut self) -> Option<u32> {
        match self.0 {
            None => {
                self.0 = Some(self.1.start_page);
            }
            Some(r) => {
                let x = self.1.next_page_no(r);

                if x == 0 {
                    return None;
                };

                self.0 = Some(x);
            },
        };

        self.0
    }
}

// TODO Move following to own module
use num::Unsigned;

/// Compiler guaranteed positive integers
// TODO Implement Copy?
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

    quickcheck! {
        fn prop_onleaf_new(x: u32) -> TestResult {
            use num::ToPrimitive;

            let once = OnLeaf::new(x);
            let back = match once.to_u32() {
                Some(_r) => x,
                None => return TestResult::discard(),
            };

            let twice = OnLeaf::new(back);

            TestResult::from_bool(once == twice)
        }
    }

    #[test]
    fn test_next_page() {
        let test_ps = PageProps::new(&NonZero(19));

        assert_eq!(11, test_ps.next_page_no(10));
        assert_eq!(12, test_ps.next_page_no(11));
        assert_eq!(09, test_ps.next_page_no(12));
        assert_eq!(08, test_ps.next_page_no(09));
        assert_eq!(13, test_ps.next_page_no(08));

        let pages0: Vec<u32> = vec![10,11,12,9,8,13,14,7,6,15,16,5,4,17,18,3,2,19,1];
        let pages1: Vec<u32> = vec![11,12,9,8,13,14,7,6,15,16,5,4,17,18,3,2,19,20,0];
        let next_pages: Vec<u32> = pages0.iter()
            .map(|x| test_ps.next_page_no(*x)).collect();

        assert_eq!(pages1, next_pages);
    }

    #[test]
    fn test_gen_pages() {
        let test_ps = PageProps::new(&NonZero(19));
        assert_eq!(
            vec![10,11,12,9,8,13,14,7,6,15,16,5,4,17,18,3,2,19,20,1],
            PageList::new(&test_ps).collect::<Vec<u32>>()
        )
    }

    #[test]
    fn test_print_order() {
        let test_ps = PageProps::new(&NonZero(19));
        let test_out = PageList::new(&test_ps).print_order().collect::<Vec<_>>();
        assert_eq!(
            vec![
                Some(10),
                Some(11),
                Some(12),
                Some(9),
                Some(8),
                Some(13),
                Some(14),
                Some(7),
                Some(6),
                Some(15),
                Some(16),
                Some(5),
                Some(4),
                Some(17),
                Some(18),
                Some(3),
                Some(2),
                Some(19),
                None,
                Some(1)],
            test_out
        )
    }
}

