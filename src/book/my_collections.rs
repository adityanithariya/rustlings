use std::{collections::HashMap, iter::zip};

#[allow(dead_code)]
pub fn my_collections() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    match v.get(1) {
        Some(i) => println!("Element at index 1: {}", i),
        None => println!("Element not found"),
    }
    for i in &v {
        print!("{i}, ");
    }
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        print!("{i}, ");
    }

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let game = format!("{s1}-{s2}-{s3}");
    println!("{game}");

    for (c, b) in zip("aditya".chars(), "aditya".bytes()) {
        println!("{c}: {b}");
    }

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("India"), 200);
    scores.insert(String::from("England"), 20);
    println!(
        "Score of India: {}",
        scores.get("India").copied().unwrap_or(0)
    );
}
