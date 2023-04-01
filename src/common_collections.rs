use std::fmt::format;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];
    v.push(1);
    // let does_not_exist = &v2[100];
    let exists = &v2[1];
    v2.push(4);
    // println!("{}", exists);
    let does_not_exist1 = v2.get(100);
    println!("{}", v2.get(2).unwrap());
    println!("{}", &v2[2]);
    let third = v2.get(2);
    match third {
        Some(value) => println!("The third elements is {}.", value),
        None => println!("No value found at third."),
    }

    for i in &mut v2 {
        println!("{}, {}, {}", i, *i, &i);
        println!("{}, {}, {}", i, *i + 10, &i);
        // println!("{}, {}, {}", i, *i, &i + 10);
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);

    let data = "initial contents";
    let s = data.to_string();

    let mut s = "initial content".to_string();
    let s9 = "new string";
    s.push_str("string");
    s.push_str(s9);
    // push_str doesn't take ownership which is why s9 can be used here
    println!("{} , {}", s, s9);

    let s10 = s + s9;
    println!("{s10}");

    let a1 = String::from("tic");
    let a2 = String::from("tac");
    let a3 = String::from("toe");

    // let b = a1 + "-" + &a2 + "-" + &a3;
    // println!("{}", b);
    let c = format!("{a1}-{a2}-{a3}");
    println!("{}", c);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
