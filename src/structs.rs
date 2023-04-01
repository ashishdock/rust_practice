// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 60,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 30,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect 2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect 3? {}", rect1.can_hold(&rect3));
    println!("The area of rectangle using method is {}.", rect1.area());

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect1)
    // );

    // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the expression’s value, the width field will get the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect1, so we use a reference to rect1 in the next call. Here’s what the output of this example looks like:
    dbg!(&rect1);
    // println!("rect is {:#?}", rect1);

    let sqr = Rectangle::square(9);
    println!("Square: {}", sqr.area());
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.height * rectangle.width
// }
