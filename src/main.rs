// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // fn route(ip_kind: IpAddrKind) {}

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    dbg!(home);
    dbg!(loopback);

    let some_number = Some(5);
    let some_char = Some('e');

    // Give some variable a null value in Rust
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
    println!("{}", sum);

    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    let five = Some(5);
    let six = plus_one(five);
    println!("Six: {}", six.unwrap());
    let none = plus_one(None);
    // println!("None: {}", plus_one(none).unwrap()) // Results in panic

    let config_max = Some(3u8);
    let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum value is configured to be {}.", max)
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
