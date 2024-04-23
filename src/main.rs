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

    // u can use clone if necessary but it's not recommended due to cost expensive
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // but we doesn't have to use clone if we store in stack
    let x = 5; // type integer is known size at compile time and store completely in stack
    let y = x;
    println!("x = {}, y = {}", x, y);

    // copy, simple scalar type can be copied and tuple only if they contain type that implement copy
    // copy tuple sample
    let t = (1, 2);
    let t2 = t;
    println!("{}, {}", t.0, t.1);
    println!("{}, {}", t2.0, t2.1);
    // copy char sample
    let c = 'c';
    let c2 = c;
    println!("{}", c);
    println!("{}", c2);

}
