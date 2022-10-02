use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File:: create("foo.md")?;

    file.write_all(b"Hello, Markdown file!")?;

    Ok(())
}