fn main() {
    
    let s = "Hello";

    let mut ss = String::from("hello"); // 当调用from函数，函数回请求自己需要的内存空间。也就是程序员来发起对内存的分配请求。

    ss.push_str("world!");

    println!("{},{}",s,ss);
}
