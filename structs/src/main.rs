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
}



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


