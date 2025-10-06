fn main() {
    stack_and_heap();
    function_ownership();
}


fn stack_and_heap() {
let stack_a = 5;
let stack_b = stack_a;
println!("stack_a: {}, stack_b: {}", stack_a, stack_b);

let heap_a = String::from("Hello");
let heap_b = heap_a;
println!("heap_b: {}", heap_b);
}


fn takes_ownership(s: String) {
    println!("{}", s);
} // s はここでスコープを抜けて drop される

fn makes_copy(x: i32) {
    println!("{}", x);
} // x はここでスコープを抜けるが、特に問題なし

fn function_ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // ここでエラー： s はムーブされた後なので使えない

    let x = 5;
    makes_copy(x);
    println!("{}", x); // 問題なく動く
}