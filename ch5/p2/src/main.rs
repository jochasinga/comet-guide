extern crate nix;
use nix::unistd::{fork, ForkResult, getpid};
use nix::sys::wait::{wait, WaitStatus};
use std::process::exit;

fn main() {
    println!("hello world (pid:{})", getpid());
    match fork() {
        Ok(ForkResult::Parent { .. }) => {
            match wait() {
                Ok(WaitStatus::Exited(pid, code)) => {
                    println!("hello, I am parent of {} (rc_wait:{}) (pid:{})", pid, code, getpid());
                },
                _ => ()
            }
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
