// 6.3.1
// 修复代码中的错误，不要新增代码行!
// 一个切片是16个字节的大小

// fn main() {
//     let arr = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2];

//     let s2: &str = "hello, world";

//     println!("{:?} {:?} {:?}",arr,s1,s2);
// }

// 6.3.2
// use std::mem;
// fn main() {
//     let arr: [char; 5] = ['中', '国', '人', '可', '能'];
//     let arr2: [char; 2] = ['中', '国'];
//     let slice = &arr[..2];
    
//     // 修改数字 `6` 让代码工作
//     // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： 因为'中'和'国'是 UTF-8 字符，它们每个占用 3 个字节，2 个字符就是 6 个字节
//     assert!(std::mem::size_of_val(&slice) == 16);
//     println!("{} {} {} {}",std::mem::size_of_val(&slice), std::mem::size_of_val(slice), std::mem::size_of_val(&arr), std::mem::size_of_val(&arr2));
// }

// 6.3.3

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // 填空让代码工作起来
//     let slice: &[i32] = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);
//     println!("{:?}",slice);
// }

// 6.3.4
// fn main() {
//     let s = String::from("hello");

//     let slice1 = &s[0..2];
//     // 填空，不要再使用 0..2
//     let slice2 = &s[..2];

//     assert_eq!(slice1, slice2);
// }
// 6.3.5

// fn main() {
//     let s = "你好，世界";
//     // 修改以下代码行，让代码工作起来
//     let slice = &s[0..3];

//     assert!(slice == "你");
// }
// 6.3.6

// 修复所有错误
fn main() {
    let mut s = String::from("hello world");

    // 这里, &s 是 `&String` 类型，但是 `first_word` 函数需要的是 `&str` 类型。
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear(); // error!

    
}
fn first_word(s: &str) -> &str {
    &s[..1]
}

  

