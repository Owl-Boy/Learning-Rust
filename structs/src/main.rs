// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
//
// fn main() {
//     let mut user1 = User {
//         email: String::from("sharmashubh223@gmail.com"),
//         username: String::from("owlyy"),
//         active: true,
//         sign_in_count: 1,
//     };
//
//     let name = user1.username;
//     user1.username = String::from("Owlyy");
//
//     let user2 = build_user(String::from("h.malfoy"), String::from("Hershey"));
//
//     let user3 = User {
//         email: String::from("owl-boy@gmail.com"),
//         username: String::from("Owly"),
//         ..user2
//     };
// }
//
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        length: 50,
        width: 30,
    };

    let rect1 = Rectangle {
        length: 40,
        width: 20,
    };

    let rect2 = Rectangle {
        length: 60,
        width: 40,
    };

    let rect3 = Rectangle::square(30);

    println!("rect {:#?}", rect);
    println!("square {:#?}", rect3);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!(
        "The area of the reactangle in square units is {} square pixels.",
        rect.area()
    )
}
