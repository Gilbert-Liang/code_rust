// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();
//     // 读取控制台的输出
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!(
//         "The value of the element at index {} is: {}",
//         index, element
//     );

//     let a1: [i32; 5] = [1,2,3,4,5];
//     let slice: &[i32] = &a1[1..3];
//     assert_eq!(slice,&[2,3]);
// }

// fn main() {
//     // 编译器自动推导出one的类型
//     let one             = [1, 2, 3];
//     // 显式类型标注
//     let two: [u8; 3]    = [1, 2, 3];
//     let blank1          = [0; 3];
//     let blank2: [u8; 3] = [0; 3];
  
//     // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
//     let arrays: [[u8; 3]; 4]  = [one, two, blank1, blank2];
  
//     // println!("{:?}",arrays);
//     // 借用arrays的元素用作循环中
//     for a in &arrays {
//       print!("{:?}: ", a);
//       // 将a变成一个迭代器，用于循环
//       // 你也可以直接用for n in a {}来进行循环
//       for n in a.iter() {
//         print!("\t{} + 10 = {}", n, n+10);
//       }
  
//       let mut sum = 0;
//       // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
//       for i in 0..a.len() {
//         sum += a[i];
//       }
//       println!("\t({:?} = {})", a, sum);
//     }
//   }

// fn main() {
//     let condition = true;
//     let number = if condition {
//         5
//     } else {
//         "six"
//     };

//     println!("The value of number is: {}", number);
// }


// #![allow(unused)]
// fn main() {
//     let mut v = 0;
//     for i in 1..10 {
//         v = if i == 9 {
//             continue
//         } else {
//             i
//         }
//     }
//     println!("{}", v);
// }

// fn main() {
//     let n = 6;
//     if n % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if n % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if n % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// for while loop   

// fn main() {
//     for i in 1..=5 {
//         println!("{}",i);
//     }

    // for item in &container {

    // }

    // for item in &mut collection {
        //todo可以在循环中修改
    //}
    // 注意container实现copy trait非常重要，如果实现了copy trait则不会发生所有权转移，负责会发生所有权转移，因此需要使用引用。
// }
//for 使用集合的引用形式，所有权会被转移到for的语句块中。

// fn main() {
//     let a = [4,3,2,1];
//     for (i,v) in a.iter().enumerate() {
//         println!("{}th element is {}",i+1,v);
//     }
// }

// fn main() {
//     for _ in 0..10{
//         println!("test");
//     }
// }

// fn main() {
//     for i in 1..4 {
//         if i == 2 {
//             continue;//break;
//         }
//         println!("{}",i);
 
//     }
// }

// fn main() {
//     let mut n = 0;

//     while n <= 5 {
//         println!("{}",n);

//         n = n + 1;
//     }

//     println!("out");
// }

// fn main() {
//     let mut n = 0;

//     loop {
//         if n > 5 {
//             break
//         }
//         println!("{}",n);
//         n+=1;
//     }
//     println!("out");
// }

fn main() {
    // let a = [10,20,30,40,50];

    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index = index + 1;
    // }

    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // loop {
    //     println!("again!");
    // }
    
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);

    
}