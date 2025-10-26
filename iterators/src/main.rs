fn main() {

}


fn iterators() {
    let v = vec![1, 2, 3, 4, 5];
    let v_iter = v.iter();
    for i in v_iter {
        println!("i: {}", i);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // you can't access v1_iter after the loop because it's consumed
    println!("v1_iter: {:?}", v1_iter);
}
