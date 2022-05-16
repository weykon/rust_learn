fn main() {
    
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {   // 此处报错了，这里已经知道s使用过，已经废弃，所以引用s已经没有内容
    let s = String::from("hello");
    &s
}