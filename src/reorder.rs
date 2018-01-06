use num::Unsigned;
use num::Zero;
use std::iter::Map;

#[derive(Debug, FromPrimitive, ToPrimitive, Clone, Copy, PartialEq)]
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
    pages: u32,
    new_pages: u32,
    start_page: u32,
    blanks: OnLeaf,
}

impl PageProps {
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

    pub fn next_page_no(&self, page: u32) -> u32 {
        let half = if page > self.start_page {
            Half::Latter
        } else {
            Half::Former
        };
        next_page_no(
            self.blanks,
            self.new_pages,
            half,
            page,
            )
    }

    pub fn print_order<'a>(&'a self) -> Box<Iterator<Item = Option<u32>> + 'a>
    {
        PageList::new(self).print_order()
    }
}

#[derive(Debug)]
pub struct PageList<'a>(Option<u32>, &'a PageProps);

impl<'a> PageList<'a> {
    pub fn new(pp: &'a PageProps) -> PageList<'a> {
        PageList(None, pp)
    }

    // No need for boxes, but can't be implemented yet
    //pub fn print_order<F>(self) -> impl Iterator<Item = Option<u32>>
    pub fn print_order(self) -> Box<Iterator<Item = Option<u32>> + 'a>
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

fn next_page_no(blanks: OnLeaf, pages: u32, half: Half, page: u32) -> u32 {
    use self::Half::*;

    match (half, page % 2) {
        (Former,0) => pages - page + 1,
        (Latter,0) => pages - page + 1,
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

