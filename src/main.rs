#![allow(unused)]
fn main() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{self, SeekFrom};

    // a library function we've written
    fn write_ten_bytes_at_end<W: Write + Seek>(writer: &mut W) -> io::Result<()> {
        writer.seek(SeekFrom::End(-10))?;

        for i in 0..10 {
            writer.write(&[i])?;
        }

        // all went well
        Ok(())
    }

    fn foo() -> io::Result<()> {
        // Here's some code that uses this library function.
        //
        // We might want to use a BufReader here for efficiency, but let's
        // keep this example focused.
        let mut file = File::create("foo.txt")?;

        write_ten_bytes_at_end(&mut file)?;
        Ok(())
    }

    // now let's write a test
    #[test]
    fn test_writes_bytes() {
        // setting up a real File is much slower than an in-memory buffer,
        // let's use a cursor instead
        use std::io::Cursor;
        let mut buff = Cursor::new(vec![0; 15]);

        write_ten_bytes_at_end(&mut buff).unwrap();

        assert_eq!(&buff.get_ref()[5..15], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
