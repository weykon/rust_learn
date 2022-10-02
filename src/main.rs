use std::{fs::File, io::Read};


fn main() -> std::io::Result<()> {
    let mut open_file = File::open(
        "./src/Javascript - Objects - Writable, Configurable & Enumerable.md",
    )?;
    let mut read_post_string = String::new();
    open_file.read_to_string(&mut read_post_string);
    println!("{}", read_post_string);
    Ok(())
}
