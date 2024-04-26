enum Cash {
    OneRupee(u32),
    TwoRupee(u32),
    TenRupee(u32),
    TwentyRupee(u32),
    FiftyRupee(u32),
}

#[allow(dead_code)]
pub fn if_let() {
    let wallet = vec![
        Cash::OneRupee(4),
        Cash::TwoRupee(2),
        Cash::TenRupee(1),
        Cash::TwentyRupee(3),
        Cash::FiftyRupee(1),
    ];

    let mut total_cash = 0u32;
    for i in wallet.iter() {
        if let Cash::OneRupee(n) = i {
            total_cash += n;
        } else if let Cash::TwoRupee(n) = i {
            total_cash += n * 2;
        } else if let Cash::TenRupee(n) = i {
            total_cash += n * 10;
        } else if let Cash::TwentyRupee(n) = i {
            total_cash += n * 20;
        } else if let Cash::FiftyRupee(n) = i {
            total_cash += n * 50;
        } else {
            break;
        }
    }
    println!("Total cash: {total_cash}");
}
