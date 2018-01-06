extern crate booklet;

use booklet::{NonZero,PageProps,PageList};

fn main() {
    match test_pages() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn test_pages() -> Result<(), &'static str> {
    let ps = NonZero::new(19).ok_or("Is zero")?;
    let pp = PageProps::new(&ps);
    let mut pl = PageList::new(&pp);

    println!("{:?}", pl.next());
    Ok(())
}
