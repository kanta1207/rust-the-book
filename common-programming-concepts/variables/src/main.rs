fn main() {
    immutable();
    mutable();
    shadowing();
}

fn immutable() {
    let x = 5;
    println!("The value of x is: {}", x);
}

// You can change the value of the variable
fn mutable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// You can change both the value and the type of the variable
fn shadowing() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}