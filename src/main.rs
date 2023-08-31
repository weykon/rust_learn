use std::net;

struct UDP {
    socket: net::UdpSocket,
}

trait Unit {
    type NewBackType;
    fn new() -> Self::NewBackType;
}

impl Unit for UDP {
    type NewBackType = Box<Option<net::UdpSocket>>;

    // 在这里我们制定对于Tcp在Unit的时候的new函数, 一般是产出 -> UDP 就好了，不过我产出了UdpSocket
    fn new() -> Self::NewBackType {
        let socket = net::UdpSocket::bind("127.0.0.1:12345").ok();
        Box::new(socket)
    }
}
// 这个NewBackType 指定了到Self的时候，具体到哪个类型再去使用。

// 分出了一个新分支作为example， 作为此时的一个复习和实践推进。
// 除了基本语法，阅读的速度，还要注重运用的思想

// 现在清晰了一点是，在于rust中，struct最为首要作为数据形式如何存在，
// 然后作为impl乃至接trait的impl，都是为数据如何操作的问题。
// 然后从这两点中去，套入一些思想，比如例子

struct ClothData {
    data: Vec<u8>,
}
struct PropertyData {
    data: Vec<i32>,
}

trait ViewData<'a> {
    type UnknowTypeWillImplThenKnow;
    fn print(&'a self) -> Self::UnknowTypeWillImplThenKnow;
}
impl<'a> ViewData<'a> for ClothData {
    type UnknowTypeWillImplThenKnow  = &'a Vec<u8>;

    fn print (&'a self) -> Self::UnknowTypeWillImplThenKnow {
        // 在这里遇到了对struct的一个不熟悉(忘记要传self了)
        &self.data
    }
}
impl ViewData for PropertyData { 
    type UnknowTypeWillImplThenKnow  = &'static Vec<i32>;
    fn print (&self) -> Self::UnknowTypeWillImplThenKnow {
        &self.data
    }
}
fn main() {
    let cloth_data = ClothData { data: vec![1, 2, 3, 4, 5] };
    let property_data = PropertyData { data: vec![10, 20, 30, 40, 50] };

    let data_vec: Vec<&dyn ViewData> = vec![&cloth_data, &property_data];

    for data in data_vec {
        data.print();
    }
}
// 这里为了单纯写不同数据的不同View的时候，
// 遇到了两个问题： 
// 1. 一个是对数据，对数据的直接获取复制还是引用的问题，
//    如果是直接获取的话，会从提示加Copy，
//    如果是引用，就要有引用符合来完善，而引用就容易出现了生命周期的问题。
// 2. 生命周期问题，算是比较麻烦去每一个地方添加'a和<'a>，而且这个过程我都不会，很无助。

impl<'b> ViewData<'b> for PropertyData{
    type UnknowTypeWillImplThenKnow  = &'b Vec<i32>;
    fn print (&'b self) -> Self::UnknowTypeWillImplThenKnow {
        &self.data
    }
}

// So, 这是基本的代码行径模式
// 让我再多写几个吧
// 更复杂的生命周期场景：你可以尝试处理一些涉及到多个生命周期参数的复杂情况，比如函数的多个参数有不同的生命周期，或者返回值的生命周期依赖于多个输入参数的生命周期。
// 更复杂的trait使用：你可以尝试定义一些更复杂的trait，比如包含多个方法的trait，或者包含关联类型和默认实现的trait。
// 使用泛型：你可以尝试使用泛型来编写一些更灵活的代码。比如，你可以定义一个可以处理任何实现了某个trait的类型的函数，或者定义一个可以包含任何类型的结构体。
// 错误处理：Rust有一套强大的错误处理机制，你可以尝试学习如何使用Result和Option来处理可能的错误和空值。
// 并发编程：Rust的所有权和生命周期系统使得它在并发编程方面表现出色。你可以尝试学习如何使用Rust的线程、锁和通道来编写并发代码。
// Web开发：你也可以尝试使用Rust进行Web开发。有一些很好的Rust Web框架，比如Rocket和Actix，你可以试试看。
// 系统编程：由于Rust的性能和安全性，它也非常适合系统编程。你可以尝试学习如何使用Rust来编写操作系统或者嵌入式系统的代码。
// 开源项目贡献：如果你对某个开源项目感兴趣，你可以尝试为它贡献代码。这是提高编程技能的一个非常好的方式，也可以让你更好地理解实际的开发流程。

// 比如泛型和trait约束的特性
trait Buy {
    fn buy(&self) -> String;
}
fn main() {}


// 对于以上一大段文字中，又让我自己觉得可笑，根本在做的以后不会去注意这种这么框大的东西，虚，这人就是虚