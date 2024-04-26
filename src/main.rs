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

    // Mutable reference
    // fix change code with mutable reference
    let mut s11 = String::from("hello s11");
    change_mut(&mut s11);
    println!("{}", s11);

    // cannot borrow as mutable more than one
    let mut s12 = String::from("hello s12");

    let r1 = &mut s12;
    println!("{}", r1);
    // let r2 = &mut s12; => you cannot do this, borrowing mutable more than one time is not allowed
    // println!("{} {}", r1, r2);
    // this practice prevent data races

    // mutable but in scope, you can do it cause it's in scope
    let mut s13 = String::from("hello s13");

    {
        let r3 = &mut s13;
        println!("{}", r3);
    }

    let r4 = &mut s13;
    println!("{}", r4);

    // with mutable and immutable reference
    let mut s14 = String::from("hello s14");

    let r5 = &s14;
    let r6 = &s14;
    // let r7 = &mut s14; you can't do this cause already borrowed as immutable
    // println!("{} {} {}", r5, r6, r7);

    println!("{} {}", r5, r6);

    // code that run because end of scope
    let mut s15 = String::from("hello s15");

    let r8 = &s15;
    let r9 = &s15;
    // scope end here in println
    println!("{} {}", r8, r9);

    // so we can use s15 again as mutable
    let r10 = &mut s15;
    println!("{}", r10);

    // dangling reference
    // let reference_to_nothing = dangle(); //=> error cause dangle return nothing

    let s16 = no_dangle();
    println!("{}", s16);

    // Slice type
    // programming problem
    let mut s17 = String::from("hello s17");

    let word = first_word(&s17);
    println!("the first word is: {}", word);

    let second_word = second_word(&s17);
    println!("the second word is: {} {}", second_word.0, second_word.1);

    s17.clear();
    println!("s17 is cleared: {}, but word is not: {}", s17, word); // => problem here s17 is cleared but word is not

    // slice string, didn't work with utf8
    let s18 = String::from("hello s18");

    let hello = &s18[0..5];
    let world = &s18[6..9];

    println!("{} {}", hello, world);

    let slice = &s18[0..2];
    let slice2 = &s18[..2];

    println!("{} {}", slice, slice2);

    let len = s18.len();

    let slice3 = &s18[3..len];
    let slice4 = &s18[3..];

    let slice5 = &s18[..];
    let slice6 = &s18[0..len];

    println!("{} {} {} {}", slice3, slice4, slice5, slice6);

    // rewrite first word
    let mut s19 = String::from("hello s19");

    let word = first_word_slice(&s19);

    // s19.clear(); => error because s19 is mutable here but called as immutable on println

    println!("the first word is: {}", word);

    let second = second_word_slice(&s19);

    println!("the second word is: {}", second);

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

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world mut reference");
}

// fn dangle() -> &String { // dangle returns a reference to a String

//    let s = String::from("hello"); // s is a new String

//    &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}  

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn second_word(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();
    // search for starting index and end index of second word
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (i + 1, s.len());
        }
    }
    (s.len(), s.len())
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}