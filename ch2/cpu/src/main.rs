use std::env;
use std::process::exit;
use std::{thread, time};

fn main() {
    if env::args().len() != 2 {
        eprintln!("usage: cpu <string>");
        exit(1);
    }

    let string = env::args().nth(1).unwrap_or_else(|| "".to_string());
    let one_second = time::Duration::from_millis(1000);

    loop {
        println!("{}", string);
        thread::sleep(one_second);
    }
}
