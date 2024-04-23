fn main() {
    //String type for ownership sample using heap
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}
