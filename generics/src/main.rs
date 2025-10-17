use std::fmt::Display;

fn main() {
    println!("Hello, world!");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // let char_result = largest_generic(&char_list);
    // println!("The largest char with generic is {}", char_result);

    // let number_result = largest_generic(&number_list);
    // println!("The largest number with generic is {}", number_result);

    print_news_article();

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    notify(&article, &article);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(&article, &tweet);

    notify(&tweet, &tweet);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// This will cause a compile error because T can be any type and we can't compare all types
// fn largest_generic_with_compile_error<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// PartialOrd: 比較可能にするためのトレイト
// Copy: 値を直接取得するためのトレイト
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


pub trait Summary {
    fn summarize(&self) -> String {
        // default implementation
        String::from("(Read more...)")
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}

fn print_news_article(){
    let article = NewsArticle {
        // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        // アメリカ、ペンシルベニア州、ピッツバーグ
        location: String::from("Pittsburgh, PA, USA"),
        // アイスバーグ
        author: String::from("Iceburgh"),
        // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize())
}

// item1とitem2が異なる型でも許容する場合
pub fn notify(item1: & impl Summary, item2: & impl Summary) {
    println!("Breaking news! {}", item1.summarize());  
    println!("Breaking news! {}", item2.summarize());
}

//  item1とitem2が同じ型であることを強制する場合
// pub fn notify<T: Summary>(item1: & T, item2: & T) {
//     println!("Breaking news! {}", item1.summarize());  
//     println!("Breaking news! {}", item2.summarize());
// }

// itemにDisplayトレイトも求めるかつitem1とitem2が同じ型であることを強制する場合
//  pub fn notify<T: Summary + Display>(item1: & T, item2: & T) {
//     println!("Breaking news! {}", item1.summarize());  
//     println!("Breaking news! {}", item2.summarize());
// }


// トレイトを実装している型を返す
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// これはコンパイルエラーになる　トレイトを実装している型を返り値の型にする場合は1つの型しか指定できない
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }


struct Pair<T> {
    x: T,
    y: T,
}

// 
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


/* 関数ジェネリック
*/


// 返り値がx, yどちらの可能性もあるゆえに返り値のライフタイムがどちらになるかわからないためコンパイルエラーになる
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// ライフタイムを指定する
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn print_novel(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
// ImportantExcerptのライフタイムはfirst_sentenceのライフタイムと同じなので、
// first_sentenceを関数内で宣言してる時点で返り値として使えない
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part);
}

// importantExcerptのpartをのを関数外から受け取る場合、インスタンスを関数外にお返しできますね
// fn print_novel<'a>(sentence: & 'a str) -> ImportantExcerpt<'a> {
//     let i = ImportantExcerpt { part: sentence };
//     println!("{}", i.part); 
//     i
// }

// ライフタイム省略
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
