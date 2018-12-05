use std::process;
use std::{thread, time};

fn main() {
    // This code does not map close to the book's C equivalence
    // since that would be using unsafe code and won't be as concise.

    let mut p: isize = 0;
    // Accessing a pointer's address is safe as long as you do not dereference (ask for the value it points to)
    let p_raw = &mut p as *mut isize;
    println!("({}) address pointed to by p: {:p}", process::id(), p_raw);

    let one_second = time::Duration::from_millis(1000);
    loop {
        thread::sleep(one_second);
        p += 1;
        println!("({}) p: {}", process::id(), p);
    }
}
