#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind : IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn main() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let four2 = IpAddrKind2::V4(String::from("127.0.0.1"));

    route(four2);

    let msg = Message::Move{x : 1, y : 3};

    msg.call();    


}

fn route(ip_type : IpAddrKind2) {

    println!("{:#?}", ip_type);

}