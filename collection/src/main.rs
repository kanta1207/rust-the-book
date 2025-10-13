fn main() {
create_vec_and_push();
access_vec_element();
fn_loop_vec();
}



fn create_vec_and_push() {
let mut v = vec![1, 2, 3];
println!("v: {:?}", v);
v.push(5);
println!("v: {:?}", v);
}

fn access_vec_element() {
    let v = vec![1, 2, 3];

    let third = &v[2];
    println!("The third element is {}", third);


    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

// &v[0] を生かしたまま v.push()（= &mut v が要る)、可変参照と不変参照が1スコープに混在するとコンパイルエラー
// fn fn_with_compile_error() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[0];

//     v.push(6);

//     println!("The first element is: {}", first);
// }

fn fn_loop_vec() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);
}


// ベクタ内で複数の型を保持したい場合はenumを使う
fn fn_enum_vec() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}