#[allow(dead_code)]
pub fn variables() {
    let x: i8 = 5;
    println!("x: {x}");

    let x = x + 1; // Shadowing

    {
        let x = x * 2;
        println!("x: {x}");
    }

    #[allow(unused_assignments)]
    let mut y = x; // Move x -> y
    y = 10;
    println!("x: {x}, y: {y}");

    let _y = {
        let x = 10;
        x + 1
    };

    let spaces = "    ";
    let size = spaces.len();
    println!("Size of \"{spaces}\" is {size}");

    let _f = 10.0;
    let _x = false;

    // Tuple
    let tup: (i8, i32, f32) = (4, 500, 10.0);
    let _four = tup.0;
    let _five_hundred = tup.1;
    let _ten = tup.2;

    // Array
    let arr = [1, 2, 3, 4];
    let _a = [3; 5];

    for i in arr {
        print!("{i}, ");
    }

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
