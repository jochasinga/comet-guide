extern crate nix;
use nix::unistd::{fork, ForkResult, getpid, execvp};
use nix::sys::wait::{wait, WaitStatus};
use std::process::exit;
use std::ffi::CString;

fn main() {
    println!("hello world (pid:{})", getpid());
    match fork() {
        Ok(ForkResult::Parent { .. }) => {
            if let Ok(WaitStatus::Exited(pid, code)) = wait() {
                println!("hello, I am parent of {} (rc_wait:{}) (pid:{})", pid, code, getpid());
            }
        },
        Ok(ForkResult::Child) => {
            println!("hello, I am child (pid:{})", getpid());
            let cmd = CString::new("wc").expect("CString::new failed");
            let filename = CString::new("src/main.rs").expect("CString::new failed");
            execvp(&cmd, &[cmd.clone(), filename]).unwrap();
        },
        Err(_) => {
            eprintln!("fork failed");
            exit(1);
        }
    }
}
