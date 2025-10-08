fn main() {
    stack_and_heap();
    function_ownership();
    reference();
    mut_reference();
    slice();
    let first_word_by_bytes = first_word_by_bytes("hello world");
    let first_word_by_chars = first_word_by_chars("hello world");
    println!("first_word_by_bytes: {}", first_word_by_bytes);
    println!("first_word_by_chars: {}", first_word_by_chars);
    array_slice();
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

// fn return_value_and_scope() {
//     let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1にムーブする

//     let s2 = String::from("hello");     // s2がスコープに入る

//     let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる
// } // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
//   // 何も起きない。s1もスコープを抜け、ドロップされる。

// fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
//                                              // 呼び出した関数にムーブする

//     let some_string = String::from("hello"); // some_stringがスコープに入る

//     some_string                              // some_stringが返され、呼び出し元関数に
//                                              // ムーブされる
// }

// // takes_and_gives_backは、Stringを一つ受け取り、返す。
// fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

//     a_string  // a_stringが返され、呼び出し元関数にムーブされる
// }


fn reference() {
 let s1 = String::from("hello");
 let (s, len) = calculate_length(s1);
 println!("The length of '{}' is {}.", s, len);

 let s2 = String::from("hello");
 let len = calculate_length_with_reference(&s2);
 println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    // 呼び出し元に所有権を返すためにs返さなあかん
 (s, len)
}

// 呼び出し元に所有権を返さないために参照を使う
fn calculate_length_with_reference(s: &String) -> usize {
    let length = s.len();
    length
}

fn mut_reference() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// fn dangling_reference() -> &String {
//  let reference_to_dropped_heap = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//  &s
// }

fn slice() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {}", hello);
    println!("world: {}", world);
}

fn first_word_by_bytes(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_by_chars(s: &str) -> &str {
   for(i,c) in s.char_indices() {
    if c == ' ' {
        return &s[..i];
    }
   }
   &s[..]
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    println!("a: {}", a[0]);
    println!("a: {}", a[1]);
    println!("a: {}", a[2]);
    println!("a: {}", a[3]);
    println!("a: {}", a[4]);
    let slice = &a[..];
    println!("slice: {}", slice[0]);
    println!("slice: {}", slice[1]);
    println!("slice: {}", slice[2]);
    println!("slice: {}", slice[3]);
    println!("slice: {}", slice[4]);

    let mut b = [1, 2, 3, 4, 5];
    let slice2 = &mut b[..];
    slice2[0] = 10;
    println!("b: {}", b[0]);
    println!("b: {}", b[1]);
    println!("b: {}", b[2]);
    println!("b: {}", b[3]);
    println!("b: {}", b[4]);
}


