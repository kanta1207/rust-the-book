use sd::collections::HashMap;


fn main() {
    println!("Hello, world!");

    first_hashmap();
    access_hashmap();
}

    


fn first_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    println!("scores: {:?}", scores);
}

fn access_hashmap() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score: {:?}", score);
}

fn entry_only_if_not_exists() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Blue")).or_insert(10);
    scores.entry(String::from("Red")).or_insert(50);
    println!("scores: {:?}", scores);
}
