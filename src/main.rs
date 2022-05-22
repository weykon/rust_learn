struct User {
    username: String,
    email: String,
}

enum IpAddKind {
    V4,
    V6,
}

enum IpAddr {
    V4(Ipv4Addr),
}
struct Ipv4Addr {
    // ...
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 匿名结构体
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
    }
}
fn build_user_same_prop_shorthadn(email: String, username: String) -> User {
    User { email, username }
}

fn main() {
    let user1 = User {
        username: String::from("weykon"),
        email: String::from("weykongkong@gamil.com"),
    };
    let user2 = build_user(String::from("abc"), String::from("dac"));

    let user3 = User {
        username: String::from("ok_man"),
        ..user1
    };

    let rect1 = Rectangle {
        width: 1,
        height: 2,
    };

    area(&rect1);
    println!("rect1 is {:?}!", rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
