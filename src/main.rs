use std::fs::File;
use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::io::Seek;

fn write_file<W: Write + Seek>(writer: &mut W) -> io::Result<()> {
    writer.seek(SeekFrom::End(-10))?;

    for i in 0..10 {
        writer.write(&[i])?;
    }

    Ok(())
}

fn example() {
    let mut file = File::create("foo.md");

    write_file(&mut file)?;
}



#[test]
fn test_writes_bytes(){
    use std::io::Cursor;
    let mut buff = Cursor::new(vec![0;15]);

    write_file(&mut buff).unwrap();

    assert_eq!(&buff.get_ref()[5..15], &[0,1,2,3,4,5,6,7,8,9]);
}

fn main(){
    
}