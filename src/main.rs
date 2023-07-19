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

fn main() {}
