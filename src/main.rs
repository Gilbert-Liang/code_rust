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
fn main() {
    let arr: [char; 5] = ['中', '国', '人', '可', '能'];
    let arr2: [char; 2] = ['中', '国'];
    let slice = &arr[..2];
    
    // 修改数字 `6` 让代码工作
    // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： 因为'中'和'国'是 UTF-8 字符，它们每个占用 3 个字节，2 个字符就是 6 个字节
    assert!(std::mem::size_of_val(&slice) == 16);
    println!("{} {} {} {}",std::mem::size_of_val(&slice), std::mem::size_of_val(slice), std::mem::size_of_val(&arr), std::mem::size_of_val(&arr2));
}


