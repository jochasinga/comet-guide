// Now write a program that uses wait() to wait for the child process to finish
// in the parent. What does wait() return? What happens if you use wait()
// in the child?

extern crate nix;

use nix::unistd::{fork, ForkResult};
use nix::sys::wait::{wait, WaitStatus};
use std::process::exit;

fn main() {
    match fork() {
        Ok(ForkResult::Child) => {
            // Waiting in the child process will wait forever
            // if let Ok(WaitStatus::Exited(_, _)) = wait() {
            //     println!("hello from child");
            // }
            println!("hello from child");
        }
        Ok(ForkResult::Parent { .. }) => {
            if let Ok(WaitStatus::Exited(_, _)) = wait() {
                println!("goodbye from parent");
            }
        }
        Err(_) => {
            eprintln!("Fork failed");
            exit(1);
        }
    }
}

