// 2.4.2
// fn main() {
//     let tup = (500, 6.4, 1);

//     // let (x, y, z) = tup;
//     let firve_hundred = tup.0;
//     let six_point_four = tup.1;

//     let one = tup.2;


//     println!("The value of y is: {} {} {}", firve_hundred,six_point_four,one);
// }

// fn main() {
//     let s1 = String::from("hello");

//     let (s2,len) = calculate_length(s1);

//     println!("The length of '{}' is {}",s2,len);
// }

// fn calculate_length(s: String) -> (String,usize) {
//     let length = s.len();

//     (s, length)
// }

#![allow(unused)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email:String::from("another@example.com"),
        username:String::from("someusername234"),
        active:user1.active,
        sign_in_count:user1.sign_in_count,
    };
    let mut user3 = User {
        email: String::from("another1@example.com"),
        ..user1
    };

    user3.username = String::from("last");
    println!("{:?}",user1.email);
    println!("{:?}",user2);
    println!("{:?}",user3);
}


