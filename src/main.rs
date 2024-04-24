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

    //ownership and function
    let sf = String::from("hello");
    println!("{}", sf);
    takes_ownership(sf);
    // calling this function and pass sf cannot be used again, takes_ownership(sf); String value move to function and can't be used again, this code error

    let i = 5;
    makes_copy(i);
    println!("{}", i);
    makes_copy(i); // for this type we can call the function and pass i value again and it's ok because i is copy (integer type)

    // return value and scope
    let s5 = gives_ownership();
    println!("{}", s5);

    let s6 = String::from("hello s6");

    let s7 = takes_and_gives_back(s6);
    println!("{}", s7);

    // use tuple
    let s8 = String::from("hello s8");
    let (s9, len) = calculate_length(s8);
    println!("The length of '{}' is {}.", s9, len);

    // Reference and Borrowing
    // Reference is a pointer to a memory location that allow you to access a value without taking ownership
    let s10 = String::from("hello s10");
    let len = calculate_length2(&s10); // pass a reference to s10 (&s10)
    println!("The length of '{}' is {}.", s10, len);

    // Because we only borrow s10, s10 cannot be modified
    // change(&s10); => error if u try to change s10 that we borrowed
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello owner");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}   

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// fn with reference &String, and with reference we won't need to give back ownership because we never take ownership and just borrow
fn calculate_length2(s: &String) -> usize {
    s.len()
}

// fn with reference and try to changes
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }