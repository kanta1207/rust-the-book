use rand::Rng;


fn main() {
print_ip_kind();
process_message(Message::Quit);
process_message(Message::Move { x: 10, y: 20 });
process_message(Message::Write(String::from("Hello, world!")));
process_message(Message::ChangeColor(255, 255, 255));


process_quit(QuitMessage);
process_move(MoveMessage { x: 10, y: 20 });
process_write(WriteMessage(String::from("Hello, world!")));
process_change_color(ChangeColorMessage(255, 255, 255));

print_cent_value();

print_plus_one();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn print_ip_kind(){
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    route(home.kind);
    route(loopback.kind);
}

fn route(ip_kind: IpAddrKind){
    println!("ip_kind: {:?}", ip_kind);
}

// Actually IpAddr is defined in https://doc.rust-lang.org/std/net/enum.IpAddr.html


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn process(&self) {
        println!("Message: {:?}", self);
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message!"),
        Message::Move { x, y } => println!("Move to {x}, {y}"),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: {r}, {g}, {b}"),
    }
}



#[derive(Debug)]
struct QuitMessage; // ユニット構造体
#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct WriteMessage(String); // タプル構造体
#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32); // タプル構造体

fn process_quit(msg: QuitMessage) { 
    println!("Quit message!");
    println!("message: {:?}", msg);
}
fn process_move(msg: MoveMessage) {
    println!("Move message!");
    println!("message: {:?}", msg);
}
fn process_write(msg: WriteMessage) {
    println!("Write message!");
    println!("message: {:?}", msg);
}
fn process_change_color(msg: ChangeColorMessage) {
    println!("Change color message!");
    println!("message: {:?}", msg);
}

// 標準ライブラリに定義されてるOption<T> nullの代用

// fn option_error(){
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

// yがNoneの可能性を孕むのでコンパイルエラー
//     let sum = x + y;
// }


#[derive(Debug)]
enum UsState {
    Alabama,
    California,
}


enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


fn print_cent_value() {
let coin = Coin::Quarter(UsState::California);
let coin2 = Coin::Penny;
let coin3 = Coin::Quarter(UsState::Alabama);
let coin4 = Coin::Dime;
let coin5 = Coin::Nickel;
println!("value of coin is {}", value_in_cents(coin));
println!("value of coin2 is {}", value_in_cents(coin2));
println!("value of coin3 is {}", value_in_cents(coin3));
println!("value of coin4 is {}", value_in_cents(coin4));
println!("value of coin5 is {}", value_in_cents(coin5));
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let random = random_some_or_none();
    println!("six: {:?}", six);
    println!("none: {:?}", none);
    println!("random: {:?}", random);
}

fn random_some_or_none() -> Option<i32> {
    let mut rng = rand::rng();
    let random = rng.random_range(0..10);
    if random % 2 == 0 {
        Some(random)
    } else {
        None
    }
}

// enumを網羅していないとコンパイルエラーを出してくれる
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }


// if letでのOption<T>の簡潔な制御フロー。matchよりも簡潔に書けるが、網羅チェックはしてくれない
// fn print_if_let() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("Max is {}", max);
//     }
// }

