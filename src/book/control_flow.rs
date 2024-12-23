use std::io::{self, Write};

#[allow(dead_code)]
pub fn control_flow() {
    let mut age = String::new();

    print!("Enter your age: ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let age = age.trim().parse::<i32>().expect("Invalid Age");
    if age < 18 {
        println!("Can't vote");
    } else {
        println!("You can vote!");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut n = String::new();

    print!("Enter n: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n = n.trim().parse::<i32>().expect("Invalid Number");

    let fib = if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        let mut idx = n;
        let mut tup: (i32, i32) = (0, 1);
        loop {
            idx = idx - 1;
            let temp = tup.0 + tup.1;
            tup.0 = tup.1;
            tup.1 = temp;
            if idx == 2 {
                break temp;
            }
        }
    };
    println!("Fibonnaci no. at {n}th index: {fib}");
}
