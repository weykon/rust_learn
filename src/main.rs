struct User {
    username: String,
    email: String,
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(Ipv4Addr),
}
struct Ipv4Addr {
    // ...
}
fn route(ip_type: IpAddrKind) {}

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
    println!("rect1 is {:?}!,second one: {:?}", rect1, rect1.area());

    let rect2 = Rectangle {
        width: 2,
        height: 1,
    };

    let b1 = rect2.can_hold(&rect1);

    println!("{}", b1);

    route(IpAddrKind::V4);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        // 选择使用&self签名的原因和之前选择的&Rectangle的原因差不多：既不用获取数据的所有权，也不需要写入数据，而只需要读取数据。
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
