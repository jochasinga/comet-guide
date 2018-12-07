use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("/tmp/file")
        .unwrap()
        .write_all(b"hello world\n")?;
    Ok(())
}
