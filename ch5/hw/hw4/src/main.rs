extern crate nix;

use std::process::exit;
use std::ffi::CString;
use nix::unistd::{
    fork,
    ForkResult,
    execv,
    // execvp,
    // execve
};

// use nix::fcntl::AtFlags;
// use std::os::unix::io::AsRawFd;

fn main() {
    match fork() {
        Ok(ForkResult::Parent { .. }) => {},
        Ok(ForkResult::Child) => {
            let cmd = CString::new("/bin/ls").expect("CString::new failed");

            // `execv` takes the command as-is.
            // (Thus, `/bin/ls` will work and `ls` will panic)
            execv(&cmd, &[cmd.clone()]).expect("execv failed");

            // `execvp` inquires the PATH variable for a command without a leading slash
            // (so `ls` will work but `bin/ls` will panic)
            // execvp(&cmd, &[cmd.clone()]).expect("execvp failed");

            // `execve` takes an additional env parameter for its new process environment variables
            // execve(&cmd, &[cmd.clone()], &[CString::new("PATH=/bin").unwrap()]).expect("execve failed");

            // `execveat` is Linux-specific and only implemented for Android and Linux.
            // let mut f = File::open("/bin/ls")?;
            // let fd = f.as_raw_fd();
            // let pathname = CString::new("").expect("Cstring::new failed");
            // execveat(fd, pathname, &[cmd.clone()], &[], AtFlags.AT_EMPTY_PATH).expect("execveat failed");
        },
        // fork failed; exit
        Err(_) => {
            eprintln!("fork failed");
            exit(1);
        }
    }
}
