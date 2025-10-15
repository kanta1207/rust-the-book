use std::fs::File;
use std::io;

fn main() {
    // panic_example();
    // result_example();
    // result_unwrap_example();
    // result_expect_example();
   let f =  result_with_question_mark();
   println!("File open result: {:?}", f);
}

// fn panic_example() {
//     let v = vec![1, 2, 3];

//     v[99];
// }

// fn result_example() {
//     let f = File::open("hello.txt");
//     use std::io::ErrorKind;

//     let f = match f {
//         Ok(file) => file,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => {
//             match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => {
//                     panic!(
//                         //ファイルを作成しようとしましたが、問題がありました
//                         "Tried to create file but there was a problem: {:?}",
//                         e
//                     )
//                 },
//             }
//         },
//         Err(error) => {
//             panic!(
//                 "There was a problem opening the file: {:?}",
//                 error
//             )
//         },
//     };
//     println!("File opened successfully: {:?}", f);
// }


// unwrap will cause panic if the file is not found
// fn result_unwrap_example() {
//     use std::fs::File;
//     let f = File::open("hello.txt").unwrap();
//     println!("File opened successfully: {:?}", f);
// }

// expect will cause panic with the message if the file is not found
// fn result_expect_example() {
//     use std::fs::File;
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
//     println!("File opened successfully: {:?}", f);
// }



fn result_with_question_mark() -> Result<File, io::Error> {
    let f = File::open("hello.txt")?;
    Ok(f)
}