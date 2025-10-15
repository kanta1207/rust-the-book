fn main() {
    panic_example();
}

fn panic_example() {
    let v = vec![1, 2, 3];

    v[99];
}