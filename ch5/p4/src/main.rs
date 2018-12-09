extern crate nix;
extern crate libc;

use nix::unistd::{fork, ForkResult, getpid, execvp};
use nix::sys::wait::{wait, WaitStatus};
use std::process::exit;
use std::ffi::CString;
use libc::{open, close};
use std::os::raw::c_uint;
use libc::{
    STDOUT_FILENO,
    O_CREAT,
    O_WRONLY,
    O_TRUNC,
    S_IRWXU
};


fn main() {
    println!("hello world (pid:{})", getpid());
    match fork() {
        Ok(ForkResult::Parent { .. }) => {
            if let Ok(WaitStatus::Exited(pid, code)) = wait() {
                println!("hello, I am parent of {} (rc_wait:{}) (pid:{})", pid, code, getpid());
            }
        },
        // child: redirect standard output to a file
        Ok(ForkResult::Child) => {
            unsafe {
                close(STDOUT_FILENO);
                open(CString::new("./p4.output").unwrap().as_ptr(), O_CREAT | O_WRONLY | O_TRUNC, S_IRWXU as c_uint);
            }
            let cmd = CString::new("wc").expect("CString::new failed");
            let filename = CString::new("src/main.rs").expect("CString::new failed");
            execvp(&cmd, &[cmd.clone(), filename]).unwrap();
        },
        // fork failed; exit
        Err(_) => {
            eprintln!("fork failed");
            exit(1);
        }
    }
}
