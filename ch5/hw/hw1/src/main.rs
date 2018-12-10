// Write a program that calls `fork()`. Before calling `fork()`, have the main
// process access a variable (e.g., `x`) and set its value to something (e.g., `100`).
// What value is the variable in the child process? What happens to the variable
// when both the child and parent change the value of `x`?

extern crate nix;
use nix::unistd::{fork, ForkResult, getpid};
use nix::sys::wait::{wait, WaitStatus};
use std::process::exit;
use std::os::raw::c_int;
use std::alloc::{alloc, Layout};


fn main() {
    println!("hello from main process (pid:{})", getpid());
    unsafe {
        let layout = Layout::new::<c_int>();
        let x = alloc(layout);
        *(x as *mut c_int) = 100;

        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                if let Ok(WaitStatus::Exited(pid, code)) = wait() {
                    println!("hello again from parent process of {} (code:{}) (pid:{})", pid, code, getpid());
                    println!("parent's x = {}", *(x as *mut c_int));
                }
            },
            Ok(ForkResult::Child) => {
                println!("hello from child process (pid:{})", getpid());
                println!("child's x = {}", *(x as *mut c_int));
                *(x as *mut c_int) = 1000;
                println!("child's x = {}", *(x as *mut c_int));
            },
            Err(_) => {
                eprintln!("fork failed");
                exit(1);
            }
        }
    }
}
