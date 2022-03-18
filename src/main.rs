#![allow(unused)]
use std::fmt::Debug;
fn main() {
    let mut a = String::from("test");
    clear(&mut a);
    // dead_end();
    forever();
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn dead_end() -> ! {
    panic!("你已经到了穷途末路，崩溃吧！");
  }

fn forever() -> ! {
    loop {
      //...
    };
}