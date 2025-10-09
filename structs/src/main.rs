fn main() {
 let sample_user : User = return_sample_user();
 println!("sample_user name : {}", sample_user.username);
 println!("sample_user email : {}", sample_user.email);
 println!("sample_user sign_in_count : {}", sample_user.sign_in_count);
 println!("sample_user active : {}", sample_user.active);
 let user_taro : User = build_user(String::from("Taro"), String::from("taro@example.com"));
 println!("user_taro name : {}", user_taro.username);
 println!("user_taro email : {}", user_taro.email);     
 println!("user_taro sign_in_count : {}", user_taro.sign_in_count);
 println!("user_taro active : {}", user_taro.active);
 debug_user(user_taro);
 rectangle();
 rectangle_with_tuple();
 rectangle_with_struct();
}


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn return_sample_user() -> User {
    let user = User { username: String::from("John"), email: String::from("john@example.com"), sign_in_count: 0, active: true };
    user
}

fn build_user(username: String, email: String) -> User {
    let user = User { username, email, sign_in_count: 0, active: true };
user
}

fn debug_user(user: User) {
    println!("user: {:?}", user);
}

fn rectangle(){
    let width = 30;
    let height = 50;
    println!("The area of the rectangle is {} square pixels.", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn rectangle_with_tuple(){
    let rect = (30, 50);
    println!("The area of the rectangle with tuple is {} square pixels.", area_with_tuple(rect));
}

fn area_with_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}


struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_struct(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

fn rectangle_with_struct(){
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle with struct is {} square pixels.", area_with_struct(rect));
}

