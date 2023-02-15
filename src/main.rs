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


fn main() {}
