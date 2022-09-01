fn main() {
    let a = String::from("123321");
    // as_bytes 将字符串转换为字节数组
    let bytes = a.as_bytes().iter().take(2);

    for u in bytes {
        println!("{}", u);
    }

    // zip

    // chiain 

    // fold 

    // scan

    // map

    //skip
    
}
