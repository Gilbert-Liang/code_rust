// #[derive(Debug)]
//  struct File {
//    name: String,
//    data: Vec<u8>,
//  }

//  fn main() {
//    let f1 = File {
//      name: String::from("f1.txt"),
//      data: Vec::new(),
//    };

//    let f1_name = &f1.name;
//    let f1_length = &f1.data.len();

//    println!("{:?}", f1);
//    println!("{} is {} bytes long", f1_name, f1_length);
//  }

// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         email: "someone@example.com",
//         username: "someusername123",
//         active: true,
//         sign_in_count: 1,
//     };
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 width is {}",rect1.width);
    println!("rect1 width is {}",rect1.height);

    println!("rect1 is {:#?}",rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
}
