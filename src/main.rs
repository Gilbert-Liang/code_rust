// 填空
use std::ops::{Range, RangeInclusive};
fn main() {
    let r1 = Range{ start: 1, end: 6 };
    let r2 = RangeInclusive::new(1, 5);

    assert_eq!((1..6), r1);
    assert_eq!((1..=5),r2);
    // assert_eq!(r1,r2);
    // assert_eq!((1..6),(1..=5));
    for i in r1 {
        println!("{}",i);
    }    
    for i in r2 {
        println!("{}",i);
    }
}
