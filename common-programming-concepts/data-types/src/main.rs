fn main() {
    data_types();
    calculate();
    tuples();
    arrays();
}

fn data_types() {
    let x : f64 = 23.4;
    let y : f32 = 3.4;
    let z : i32 = 42;
    let a : bool = true;
    let b : char = '🚀';
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
}

fn calculate() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of floored is: {}", floored);
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

fn arrays() {
    let i32_array: [i32; 5] = [1, 2, 3, 4, 5];
    let i32_array2: [i32; 5] = [3; 5];
    let str_in_stack_arr: [&str; 5] = ["Hello", "World", "Rust", "Programming", "Language"];
    let str_in_heap_arr: [String; 5] = ["Hello".to_string(), "World".to_string(), "Rust".to_string(), "Programming".to_string(), "Language".to_string()];
    println!("The value of i32Array is: {}", i32_array[0]);
    println!("The value of i32Array2 is: {}", i32_array2[0]);
    println!("The value of strInStackArr is: {}", str_in_stack_arr[0]);
    println!("The value of strInHeapArr is: {}", str_in_heap_arr[0]);
}
