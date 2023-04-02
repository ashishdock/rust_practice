fn main() {
    // let v1 = vec![1, 2, 3];

    // for val in &v1 {
    //     println!("Got: {}", val);
    // }

    // let v1_iter = v1.iter();
    // println!("{:?}", v1_iter);
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| x + 1);
    let v3: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
    for val in v2 {
        println!("Got: {}", val);
    }
}
