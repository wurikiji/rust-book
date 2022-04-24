enum IpAddrKindV1 {
    V4,
    V6,
}

struct IpAddrV1 {
    kind: IpAddrKindV1,
    address: String,
}

#[derive(Debug)]
struct Ipv4Addr {}
#[derive(Debug)]
struct Ipv6Addr {}

#[derive(Debug)]
enum IpAddrKindV2 {
    V4(String),
    V4_Num(u8, u8, u8, u8),
    V4_Struct(Ipv4Addr),
    V4_Fields {
        first: u8,
        second: u8,
        third: u8,
        fourth: u8,
    },
    V6(String),
    V6_Struct(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum can also have impl
impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IpAddrKindV1::V4;
    let six = IpAddrKindV1::V6;

    let home = IpAddrV1 {
        kind: four,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddrV1 {
        kind: IpAddrKindV1::V6,
        address: String::from("::1"),
    };

    let home = IpAddrKindV2::V4(String::from("127.0.0.1"));
    let home_num = IpAddrKindV2::V4_Num(127, 0, 0, 1);
    let loopback = IpAddrKindV2::V6(String::from("::1"));
    match home {
        IpAddrKindV2::V4(ref address) => println!("V4 String addr: {}", address),
        IpAddrKindV2::V4_Num(a,b,c,d) => println!("V4 Num tuple addr: {}.{}.{}.{}",a,b,c,d),
        IpAddrKindV2::V4_Struct(ref addr) => println!("V4 struct addr: {:?}", addr),
        IpAddrKindV2::V4_Fields{
            first: a, second: b, third: c, fourth: d,
        } => println!("V4 fields addr: {}.{}.{}.{}",a,b,c,d),
        IpAddrKindV2::V6(ref addr) => println!("V6 String addr: {:?}", addr),
        IpAddrKindV2::V6_Struct(ref v6_struct) => println!("V6 Struct addr: {:?}", v6_struct),
    };

    println!("Hello, enum! {:?}", home);
    println!("loopback: {:?}", loopback)
}

fn route(ip_kind: IpAddrKindV1) {}
