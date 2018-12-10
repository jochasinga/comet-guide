use std::process;
use std::{thread, time};
use std::alloc::{alloc, Layout};
use std::os::raw::c_int;

fn main() {
    // This is unsafe code
    unsafe {
        let layout = Layout::new::<c_int>();
        let p = alloc(layout);
        println!("({}) address pointed to by p: {:p}", process::id(), p);
        *(p as *mut c_int) = 0;
        loop {
            thread::sleep(time::Duration::from_millis(1000));
            *(p as *mut c_int) += 1;
            println!("({}) p: {}", process::id(), *(p as *mut c_int));
        }
    }
}
