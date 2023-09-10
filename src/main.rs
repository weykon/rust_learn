/// 这是关于&str，由指针和长度信息组成
fn main() {
    let str = "hello world";
    let ptr = str.as_ptr();
    println!("ptr: {:p}", ptr);
    // println!("content : {} ", *ptr); 现在这样写不安全了
}
