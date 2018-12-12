// Write another program using `fork()`. The child process should print "hello";
// the parent process should print "goodbye". You should try to ensure that
// the child process always prints first;; can you do this **without** calling `wait()`
// in the parent?

extern crate nix;
extern crate libc;

use nix::unistd::{fork, ForkResult};
use std::process::exit;
// use nix::unistd::sleep;
// use libc::c_uint;

fn main() {
    match fork() {
        Ok(ForkResult::Child) => {
            println!("hello");
        }
        Ok(ForkResult::Parent { .. }) => {
            // The parent never waits for the child to say "hello" unless explicit
            // told to "wait" or "sleep"
            // sleep(1 as c_uint);
            println!("goodbye");
        }
        Err(_) => {
            eprintln!("Fork failed");
            exit(1);
        }
    }
}

