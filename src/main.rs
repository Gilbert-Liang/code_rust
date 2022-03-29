// fn main() {
//     let s = Some(String::from("Hello!"));

//     if let Some(_) = s {
//         println!("found a string");
//     }
//     println!("{:?}", s);
// }

// fn main() {
//     let numbers = (2, 4, 8, 16, 32);

//     match numbers {
//         (.., second, ..) => {
//             println!("Some numbers: {}", second)
//         },
//     }
// }

#![allow(unused)]
// fn main() {
//     let num = Some(4);

//     match num {
//         Some(x) if x < 5 => println!("less than five: {}", x),
//         Some(x) => println!("{}", x),
//         None => (),
//     }
// }

enum Message {
    Hello { id: i32 },
}




fn main() {

    let msg = Message::Hello { id: 1 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
    // let x = Some(5);
    // let y = 10;

    // match x {
    //     Some(50) => println!("Got 50"),
    //     Some(n) if n == y => println!("Matched, n = {}", n),
    //     _ => println!("Default case, x = {:?}", x),
    // }

    // println!("at the end: x = {:?}, y = {}", x, y);
    // let x = 4;
    // let y = false;

    // match x {
    //     4 | 5 | 6 if y => println!("yes"),
    //     _ => println!("no"),
    // }
}
