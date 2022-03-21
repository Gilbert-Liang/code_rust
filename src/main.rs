// use std::thread;
fn main() {
    let mut s = String::from("hello");

    change1(&mut s);
    // thread::spawn(change1);
    change2(&mut s);
    println!("{}",s);
}

fn change1(some_string:  &mut String) {
    some_string.push_str(", world");
}

fn change2(some_string:  &mut String) {
    some_string.push_str(", liangdong");
}