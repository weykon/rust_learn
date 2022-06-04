#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 3 };
    println!("{:?}", p);
}
// 这里需要对自定义的类型进行标注，才可实现打印输出的功能。

// 若想让更自定义的输出，可以使用  std::fmt::Display 特征

