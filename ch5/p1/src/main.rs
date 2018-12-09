extern crate nix;
use nix::unistd::{fork, ForkResult, getpid};
use std::process::exit;

fn main() {
    println!("hello world (pid:{})", getpid());
    match fork() {
        Ok(ForkResult::Parent { child, ..}) => {
            println!("hello, I am parent of {} (pid:{})", child, getpid())
        },
        Ok(ForkResult::Child) => {
            println!("hello, I am child (pid:{})", getpid());
        },
        Err(_) => {
            eprintln!("fork failed");
            exit(1);
        }
    }
}
