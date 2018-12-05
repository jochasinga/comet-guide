use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{env, thread};

fn main() {
    // Arc atomically count references to this resource making it possible
    // to call `counter.clone()` later.
    let counter = Arc::new(Mutex::new(0));
    let loops: isize;

    if env::args().len() != 2 {
        eprintln!("usage: threads <value>");
        exit(1);
    }

    let first_arg = env::args().nth(1).unwrap_or_else(|| "".to_string());
    loops = first_arg.parse::<isize>().unwrap();

    println!("Initial value : {}", counter.lock().unwrap());

    let mut children = Vec::new();

    for _ in 0..2 {
        let counter = counter.clone();
        let child = thread::spawn(move || {
            for _i in 0..loops {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            // the lock is unlocked here when `num` goes out of scope.
        });
        children.push(child)
    }

    for child in children {
        child.join().unwrap();
    }

    println!("Final value    : {}", counter.lock().unwrap());
}
