fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // s    将报错，这里的s已经被废弃

    let s2 = String::from("hello2");

    let s3 = takes_and_gives_back(s2);

    touple_test(s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn touple_test(a_string:String) -> (String,usize) {
    let length = a_string.len();
    (a_string,length)
}