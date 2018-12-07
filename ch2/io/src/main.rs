use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("/tmp/file")?;
    file.write_all(b"hello world\n")?;
    Ok(())
}
