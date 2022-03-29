// #![allow(unused)]
// fn main() {
//     // Vec是动态数组
//     let mut stack = Vec::new();

//     // 向数组尾部插入元素
//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     // stack.pop从数组尾部弹出元素
//     while let Some(top) = stack.pop() {
//         println!("{}", top);
//     }
// }

// fn main() {
//     let v = vec!['a', 'b', 'c'];

//     for (index, value) in v.iter().enumerate() {
//         println!("{} is at index {}", value, index);
//     }
// }

// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current location: ({}, {})", x, y);
// }

// fn main() {
//     let point = (3, 5);
//     print_coordinates(&point);
// }

// fn main() {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched, y = {:?}", y),
//         _ => println!("Default case, x = {:?}", x),
//     }

//     println!("at the end: x = {:?}, y = {:?}", x, y);
// }

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    //ChangeColor(i32,i32,i32),
    ChangeColor(Color),
}

fn main() {
    t1();
    // t2();
    t3();
    t4();
    foo(3, 4);
    t5();
    t6();
    let _x = 4;
    let _y = 10;
}

fn t1() {
    // let p = Point { x: 0, y: 7 };
    let p = Point { x: 0, y: 1 };

    match p {
        Point { x, y: 0 } => println!("x:{}", x),
        Point { x: 0, y } => println!("y:{}", y),
        Point { x, y } => println!("x:{}, y:{}", x, y),
    }
}

// fn t2() {
//     let msg = Message::ChangeColor(0, 160, 255);

//     match msg {
//         Message::Quit => {
//             println!("The Quit variant has no data to destructure.")
//         }
//         Message::Move { x, y } => {
//             println!("Move in the x direction {} and in the y direction {}", x, y);
//         }
//         Message::Write(text) => println!("Text message: {}", text),
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {}, green {}, and blue {}", r, g, b)
//         }
//     }
// }

fn t3() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
}

fn t4() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{} {} {} {}", feet, inches, x, y);
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn t5() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("outer setting is {:?}", setting_value);
}

fn t6() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}
