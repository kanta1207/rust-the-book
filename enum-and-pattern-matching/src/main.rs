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

fn option_error(){
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    let sum = x + y;
    
}
