fn main() {

    let s1 = String::from("hello world");

    let hello = &s1[0..5]; // 或者 &s1[..5];
    let world = &s1[6..11]; // 或者 &s1[6..];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // 转换成字节数组

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()

  
}
