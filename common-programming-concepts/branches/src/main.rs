fn main() {
    println!("Hello, world!");
    simple_if_else(5);
    loop_example();
    loop_label();
    while_loop();
}

fn simple_if_else(number: i32) {
    if number == 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}


fn loop_example() {
    println!("loop example");
    let mut count = 0;
    loop {
        count += 1;
        println!("count is {}", count);
        if count == 5 {
            break;
        }
    }
}

fn loop_label() {
    println!("loop label");
    let mut count = 0;
    'outer_loop: loop {
        count += 1;
        println!("count in outer loop is {}", count);
        loop {
            count += 1;
            println!("count in inner loop is {}", count);
            if count == 5 {
                break 'outer_loop;
            }
        }
        println!("this will not be printed");
       }
}

fn while_loop() {
    println!("while loop");
    let mut count = 0;
    while count < 5 {
        count += 1;
        println!("count is {}", count);
    }
}

fn for_loop() {
    println!("for loop");
    for i in 0..5 {
        println!("i is {}", i);
    }
}