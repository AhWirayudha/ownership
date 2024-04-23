fn main() {
    // String type for ownership sample using heap
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // error ownership sample
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); error caused by ownership already moved to s2
    // println!("{}, world!", s1);
    println!("{}, world! s2", s2);
}
