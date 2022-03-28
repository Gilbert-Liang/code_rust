// #[warn(dead_code)]
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South => {
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }


// #![allow(unused)]

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn main() {
//     let coin = Coin::Dime;
//     println!("{}",value_in_cents(coin));
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny =>  {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// enum IpAddr {
//     Ipv4,
//     Ipv6
//  }
 
//  fn main() {
//      // let d_panic = Direction::South;
//      let ip1 = IpAddr::Ipv6;
//      let ip_str = match ip1 {
//          IpAddr::Ipv4 => "127.0.0.1",
//          _ => "::1",
//      };
 
//      println!("{}", ip_str);
//  }


// #![allow(unused)]
// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState), // 25美分硬币
// }
// fn main() {
//     let coin = Coin::Quarter(UsState::Alabama);
//     println!("{}",value_in_cents(coin));
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//     }
// }

// enum Action {
//     Say(String),
//     MoveTo(i32, i32),
//     ChangeColorRGB(u16, u16, u16),
// }

// fn main() {
//     let actions = [
//         Action::Say("Hello Rust".to_string()),
//         Action::MoveTo(1,2),
//         Action::ChangeColorRGB(255,255,0),
//     ];
//     for action in actions {
//         match action {
//             Action::Say(s) => {
//                 println!("{}", s);
//             },
//             Action::MoveTo(x, y) => {
//                 println!("point from (0, 0) move to ({}, {})", x, y);
//             },
//             Action::ChangeColorRGB(r, g, _) => {
//                 println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
//                     r, g,
//                 );
//             }
//         }
//     }
// }

// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South => {
//             println!("South or North");
//         },
//         Direction::West => println!("West"),
//     };
// }

// fn main() {
//     let v = Some(3u8);
//     match v{
//         Some(k) => println!("{}",k),
//         _ => (),
//     }
//     if let Some(k) = v {
//         println!("{}",k);
//     }
// }
//当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    let v1 = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    for value in v1.into_iter() {
        println!("{:?}",value);
    }

    let foo = 'f';
    println!("{}",matches!(foo, 'A'..='Z' | 'a'..='z'));
    let bar = Some(4);
    println!("{}",matches!(bar, Some(x) if x > 2));

}
