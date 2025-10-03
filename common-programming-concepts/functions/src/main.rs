fn main() {
    println!("Hello, world!");
    
    another_function();
    function_with_parameters(5, 6);
    let return_value = function_with_return_value();
    println!("The return value is: {}", return_value);
}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameters(x: i32, y: i32)  {
    let sum_of_x_and_y = x + y;
    let difference_of_x_and_y = x - y;
    let product_of_x_and_y = x * y;
    let quotient_of_x_and_y = x / y;
    let remainder_of_x_and_y = x % y;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The sum of x and y is: {}", sum_of_x_and_y);
    println!("The difference of x and y is: {}", difference_of_x_and_y);
    println!("The product of x and y is: {}", product_of_x_and_y);
    println!("The quotient of x and y is: {}", quotient_of_x_and_y);
    println!("The remainder of x and y is: {}", remainder_of_x_and_y);
}

fn function_with_return_value() -> i32 {
    return 5;
}