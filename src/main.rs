fn main() {
    let str1 = "Pascal";//堆内存
    let str2 = "haskell";//堆内存
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{:p}",str1);
    println!("{:p}",str2);
    println!("{:p}",s.as_ptr());
    println!("{:p}",hello);
    println!("{:p}",world);

    // println!("{:p}",func1);
    // println!("{:p}",func2);

}

fn func1() {

}

fn func2() {

}
