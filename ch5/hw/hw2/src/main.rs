// Write a program that opens a file (with the `open()` system call) and then
// calls `fork()` to create a new process. Can both the child and parent access
// the file descriptor returned by `open()`? What happens when they are writing
// to the file concurrently, i.e., at the same time?

extern crate nix;
extern crate libc;
use nix::unistd::{fork, ForkResult, getpid};
use std::process::exit;
use std::ffi::CString;
use std::os::raw::{c_uint, c_int};
use libc::{open, fdopen, fprintf, fclose, FILE};
use libc::{
    O_CREAT,
    O_WRONLY,
    O_TRUNC,
    S_IRWXU
};

fn main() {
    println!("hello from main process (pid:{})", getpid());
    unsafe {
        let fd: c_int = open(CString::new("./hw2.file")
                                .unwrap()
                                .as_ptr()
                                , O_CREAT | O_WRONLY | O_TRUNC, S_IRWXU as c_uint);
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                // Does not wait
                println!("hello from parent process (pid:{})", getpid());
                let fptr: *mut FILE = fdopen(fd, CString::new("w").unwrap().as_ptr());
                fprintf(fptr, CString::new("World").unwrap().as_ptr());
                fclose(fptr);
            },
            Ok(ForkResult::Child) => {
                println!("hello from child process (pid:{})", getpid());
                let fptr: *mut FILE = fdopen(fd, CString::new("w").unwrap().as_ptr());
                fprintf(fptr, CString::new("Hello").unwrap().as_ptr());
                fclose(fptr);
            },
            Err(_) => {
                eprintln!("fork failed");
                exit(1);
            }
        };
    }
}
