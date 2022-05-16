fn main() {
    let s = String::from("hello");

    let mut s1 = String::from("hello1");

    change(&mut s1);

}

// fn change(s: &String) {
//     s.push_str(",world");   //此处编译错误，因为引用是默认不可变
// }

fn change(s: &mut String) {
    s.push_str(",world");
}
