use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("foo.md")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("[rust read file, here is contents: ]{}", contents);
    println!("now I change the contents ");
    let some_new_words = "--(I add some words)";
    contents.push_str(some_new_words);
    println!("{}", contents);
    Ok(())
}
