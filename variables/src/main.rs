use std::iter::Product;

fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = "six";
    println!("The value of x is {}", x);

    // const SENKU_NUMBER: u64 = 10_000_000_000;

    // Scalar DataTypes
    //  Integers
    let a = 98_222; // Decimal
    let b = 0xff; // Hexadecimal
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte

    //  Flaoting-point  Numbers
    let f: f64 = 2.0;
    let g: f32 = 3.0;

    let sum = a + b;
    let differnce = c - d;
    let product = e * d;
    let quotient = a / b;
    let remainder = c % d;

    //  Booleans
    let t = true;
    let f = false;

    //  Characters
    let c = 'c';

    // Compound DataTypes
    // Tuples
    let tup = ("How you doin'", 3);
    let (string, wc) = tup;
    let str1 = tup.0;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    let byte = [0; 8];

    let sum = my_function(11, 22);

    // Control Flow
    //  If-else
    let number = 5;
    if number < 10 {
        println!("Single digit");
    } else if number < 100 {
        println!("Double digit");
    } else {
        println!("hmmmmm that's what she said");
    }

    let condition = true;
    let number = if condition { 5 } else { 0 };

    //  loops

    let mut counter = 0;
    let result = loop {
        println!("{}", counter);

        counter += 1;
        if counter <= 20 {
            break counter;
        }
    };

    println!("The value of result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{} !!", number);
        number -= 1;
    }

    println!("LIFTOFFF");

    let a = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("The Value is {}", elem);
    }

    for number in 1..10 {
        println!("The number is {}", number);
    }

    // Single Line Comments

    /*
        Block Comments
    */
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    return x + y;
}
