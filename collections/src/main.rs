use std::collections::HashMap;

fn main() {
    // let _a = [1, 2, 3];
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    //
    // let _v2 = vec![1, 2, 3];
    //
    // let v3 = vec![1, 2, 3, 4, 5];
    //
    // let third = &v3[2];
    // // println!("The third element is {}", third);
    //
    // match v3.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element!!"),
    // };
    //
    // for i in &v3 {
    //     println!("{}", i);
    // }

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.21),
        SpreadSheetCell::Text(String::from("blue")),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an integer !!"),
    }

    // In rust string are stored as a collecton of UTF-8 encoded bytes 
    let _s1: String = String::new();
    let s2: &str = "initail contents";
    let _s3: String = s2.to_string();
    let _s4: String = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    let s_a = String::from("Hello, ");
    let s_b = String::from("world!");
    let _s_c = s_a + &s_b;

    let blue = String::from("Blue");
    let red = String::from("Red");

    let mut scores = HashMap::new();

    scores.insert(blue, 20);
    scores.insert(red, 40);

    let team_name = String::from("Blue");
    let _score = scores.get("Blue");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    };

    let text = String::from("Hello World, Beautiful World");
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
