fn main() {
    let s = String::from("hello");

    let s2 = s.clone();

    println!("{s}");
    println!("{s2}");

    let x = 5;

    takes_ownership(s);
    // println!("{s}");

    makes_copy(x);
    println!("{x}");

    let s3 = gives_ownership();

    println!("{s3}");

    let s4 = String::from("world");

    let s5 = takes_and_gives_back(s4);

    // println!("{s4}");
    println!("{s5}");

    let mut s6 = String::from("John");
    let len = calculate_length(&s6);

    let mut s7 = String::from("My");
    println!("The length of {} is {}", s6, len);

    println!("{s7}");
    mutable_ref_modify(&mut s7);

    println!("{s7}");
    let r1 = &s6;
    println!("{r1}");
    // let r3 = &mut s6; // as long as any of r1 and r2 are being used after this, it won't run
    {
        // let r2 = &s6;
        // println!("{r2}");
    }
    println!("{r1}");
    let r3 = &mut s6;
    println!("{r3}");
    println!("{s7}");
    let mut slices = String::from("Hello World");
    let slices_first = &slices[0..7];
    println!("{slices_first}");
    let word = first_word(&slices);
    // slices.clear(); // cannot use it here. will cause error
    println!("{word}");
    println!("{} has the first word as {}", slices, word);
    slices.clear();
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// This doesn't work
// fn wont_work(s: &String) {
//     s.push_str(", world!")
// }

fn mutable_ref_modify(s: &mut String) {
    s.push_str(", world")
}

// // This was wrong implementation
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }
// Correct one
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}
