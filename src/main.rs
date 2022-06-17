fn main() {
    println!("main starting...");

    // 1 -
    // let r;

    // {
    //     let x = 5;
    //     r = &x ;        // here err !
    // }

    // println!("r: {}", r);
    // ⬆️ 悬垂指针了

    // 2 -    
    // 函数中的生命周期
    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2);
    // - - - - - - - - - - - - - - - - -
}

// 2    - 
// fn longest(x: &str, y: &str) -> &str {    // 在返回的这个&str中，因为目前不知道是返回x还是y的长度，所以传入的生命周期是未知的
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// 这个时候编译器无法自动推导出生命周期，此时需要我们手动去标注
// - - - - - - - -  