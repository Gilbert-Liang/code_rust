// fn main() {
//     let age = Some(30);
//     println!("在匹配前，age是{:?}",age);
//     if let Some(age) = age {
//         println!("匹配出来的age是{}",age);
//     }
 
//     println!("在匹配后，age是{:?}",age);
//  }

fn main() {
    let age = Some(30);
    println!("在匹配前，age是{:?}",age);
    match age {
        Some(age) =>  println!("匹配出来的age是{}",age),
        _ => ()
    }
    println!("在匹配后，age是{:?}",age);
 }
 