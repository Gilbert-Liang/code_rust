use std::fmt;
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
// #[derive(Display)]
pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking new! {}", item.summarize());
// }

pub fn notify<T: Summary + fmt::Display>(item: &T) {
    println!("Breaking news! {} {}", item.summarize(), item);

}

impl fmt::Display for Weibo {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self.content)
    }
}

fn main() {
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};
    notify(&weibo);
}