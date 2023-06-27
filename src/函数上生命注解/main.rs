fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = Stirng::from("really long string");
    result.as_str()
}

// 编译失败，
// 因为变量 result 在 longest 结尾会离开作用域然后被清理，但是现在
// 返回这个引用， 所以无法指定生命周期参数来改变悬垂引用的。