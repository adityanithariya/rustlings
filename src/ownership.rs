#[allow(dead_code)]
pub fn ownership() {
    let x = 5;
    let y = x; // Primitive Types
    println!("x: {x}, y: {y}");

    let x = String::from("Hello World!");
    let y = x; // Shallow Copy: Reference to array of characters in heap is copied
               // println!("{x}"); // Will give error, as ownership of x is transferred to y, this is done to maintain concurrency
    println!("{y}");

    let mut x = String::from("Hello");
    let y = x.clone(); // Deep Copy
    println!("x: {x}, y: {y}");

    println!("Length of \"{x}\": {}", calculate_length(&x));

    modify_str(&mut x);
    println!("Modified x: {}", x);
    /*
        References:
        1. Can have more than one immutable reference at a time
        2. Can have only one mutable reference at a time
    */

    first_word(&x);
}

fn calculate_length(x: &String) -> usize {
    x.len()
}

fn modify_str(x: &mut String) {
    x.push_str(" World!");
}

/*
fn dangle() -> &String { // This is a example of dangling references of string s
    let s = String::from("Hello World");
    &s
}
*/

fn first_word(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
