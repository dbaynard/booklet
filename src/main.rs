extern crate booklet;

use booklet::{NonZero,PageProps};

fn main() {
    match test_pages() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn test_pages() -> Result<(), &'static str> {
    let size = NonZero::new(19).ok_or("Needs to be a zero")?;
    let ps = PageProps::new(&size);

    println!("{:?}", ps);

    let pages: Vec<u32> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19];
    //let pages: Vec<u32> = vec![10,11,12,9,8,13,14,7,6,15,16,5,4,17,18,3,2,19,1];

    let new_pages: Vec<u32> = pages.iter()
        .map(|x| ps.new_page_no(*x)).collect();

    println!("{:?}", new_pages);

    Ok(())
}
