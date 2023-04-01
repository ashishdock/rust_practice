const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    let y = x + 1;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    let a = 5;
    let a = a + 1;
    {
        let a = a * 2;
        println!("The value of a in inner scope is: {a}");
    }
    println!("The value of a is: {a}");
    let tup = ("string", 19, true, 99.99);
    let (b, c, d, _) = tup;
    println!("The value of b is {b}");
    println!("The value of c is {}", tup.3);

    fn five() -> i32 {
        5
    }

    println!("The value of x from function five is {}", five());

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    println!("The value of x with plus one is {}", plus_one(5));

    let condition = true;
    let e = if condition { 5 } else { 6 };
    // let e = if condition { 5 } else { "Six" };
    println!("The value of e is {}", e);

    for number in 1..=4 {
        println!("{}", number);
    }
}
