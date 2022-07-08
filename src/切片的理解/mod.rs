pub fn slice_learn() {
    let s = String::from("abcd");

    let ab = &s[0..2];
    let cd = &s[2..=3];

    println!("slice : {}, ab: {} , cd : {} ", s, ab, cd);
}
// https://www.runoob.com/rust/rust-slice.html

// s 被部分引用，禁止更改其值。
// 实际上，到目前为止你一定疑惑为什么每一次使用字符串都要这样写String::from("runoob") ，直接写 "runoob" 不行吗？
// 事已至此我们必须分辨这两者概念的区别了。
// 在 Rust 中有两种常用的字符串类型：str 和 String。
// str 是 Rust 核心语言类型，就是本章一直在讲的字符串切片（String Slice），常常以引用的形式出现（&str）。

// str 是 Rust 核心语言类型
// String 类型是 Rust 标准公共库提供的一种数据类型