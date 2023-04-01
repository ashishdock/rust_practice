


fn main() {
 let x = 5;
 println!("The value of x is: {}", x);
 let x = "String";
 println!("The value of x is: {}", x);

 const SUBSCRIBER_COUNT: u32 = 100_000;

let a: i32 = 98_222; // Decimal
let b: i32 = 0xff; // Hex
let c: i32 = 0o77; // Octal
let d: i32 = 0b1111_0000; // Binary
let e: u8 = b'A'; // Byte (u8 only)

let f: u8 = 255;

let tup = ("Let's Get Rusty", 100_000);
let (channel, sub_count) = tup;
let sub_count = tup.1;

let error_codes = [200, 404, 500];
let not_found = error_codes[1];

let byte = [0; 8];

let sum = my_function(2, 3);
println!("{}", sum);

if sum < 10 {
    println!("Sum is less than 10")
} else if sum > 10 {
    println!("Sum is greater than 10")
}

let mut counter = 0;
let result = loop {
    counter += 1;
    
    if counter == 10 {
        break counter; // this will return counter
    }
    println!("{}", counter);
};

println!("The result is {}", result);

let coll = [10, 20, 30, 40, 50];
for element in coll.iter() {
    println!("The value is {}", element);
}

for number in 1..4 {
    println!("{}", number);
}

}

fn my_function(x: i32, y: i32) -> i32{
    x + y // no need for return keyword
}