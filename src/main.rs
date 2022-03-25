// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// //不要滥用枚举
// fn main() {
//     let m1 = Message::Quit;
//     let m2 = Message::Move{x:1,y:1};
//     let m3 = Message::ChangeColor(255,255,0);
// }

#![allow(unused)]

fn main() {

}

fn new (stream: TcpStream) {
    let mut s = stream;
    if tls {
      s = negotiate_tls(stream)
    }
  
    // websocket是一个WebSocket<TcpStream>或者
    //   WebSocket<native_tls::TlsStream<TcpStream>>类型
    websocket = WebSocket::from_raw_socket(
      stream, ......)
}

enum Websocket {
    Tcp(Websocket<TcpStream>),
    TLs(Websocket<native_tls::TlsStream>;)
}