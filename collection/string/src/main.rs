fn main() {
    println!("Hello, world!");

    fn_string();
    extend_string(String::from("Good Morning"));
}


fn fn_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn extend_string(s : String) {
    s.push_str(", world!");
}

