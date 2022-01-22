use std::u8;

// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
//
// enum Message {
//     Quit,
//     Move {x: u32, y: u32},
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }
//
// impl Message {
//     fn some_func() {
//         println!("Lets, goooo!!");
//     }
// }
//
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
//
// fn main() {
//     let local_host = IpAddrKind::V4(127, 0, 0, 1);
// }
//
// fn route(ip_kind: IpAddrKind) {}

// fn main() {
//     // enum Option<T> {
//     //     Some(T),
//     //     None,
//     // }
//     //
//     // let some_number = Some(5);
//     // let some_string = Some("S string");
//     //
//     // let absent_number: Options<i32> = None;
//
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
//
//     let sum = x + y.unwrap_or(default);
// }

fn main() {
    value_in_cents(Coin::Quarter);
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!!");
            1
        }
        Coin::Dime => 5,
        Coin::Nickle => 10,
        Coin::Quarter => 25,
    }
}
