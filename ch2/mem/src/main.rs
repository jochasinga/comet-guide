use std::process;
use std::{thread, time};
use std::alloc::{alloc, Layout};

fn main() {
    // This is unsafe code
    unsafe {
        let layout = Layout::new::<isize>();
        let p = alloc(layout);
        println!("({}) address pointed to by p: {:p}", process::id(), p);
        *(p as *mut isize) = 0;
        loop {
            thread::sleep(time::Duration::from_millis(1000));
            *(p as *mut isize) += 1;
            println!("({}) p: {}", process::id(), *p);
        }
    }
}
