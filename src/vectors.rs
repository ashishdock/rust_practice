use core::panic;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    match v.get(20) {
        Some(value) => println!("The element at index {} is {}.", 2, value),
        None => println!("There is no such element"),
    }

    let s1 = String::from("Hello");
    let s2 = String::from(" World!");
    let s3: String = format!("{}{}", s1, s2);
    println!("{}", s3);
    // let c:char = hello.chars()[0];
    for g in s1.graphemes(true) {
        println!("{}", g);
    }
    panic!("Crash and burn!");
}
