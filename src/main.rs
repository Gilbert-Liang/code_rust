// 方案1
// fn display_array<T: std::fmt::Debug>(arr: &[T]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(&arr);

//     let arr: [i32;2] = [1,2];
//     display_array(&arr);
// }

// 方案2
// fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; ddN]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(arr);

//     let arr: [i32; 2] = [1, 2];
//     display_array(arr);
// }

// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// fn something<T> (val T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTruem
// {
//     something([0u8; 0]);
//     something([0u8; 512]);
//     something([0u8; 1024]);
// }

// Rust中泛型是零成本的抽象，意味着你在使用泛型时，完全不用担心性能上的问题。
// 如何选择都是权衡得失的，既然我们获得了性能上的巨大优势，那么又失去了什么呢？Rust是在编译期为泛型对应的多个类型，生成各自的代码，因此损失了编译速度和增大了最终生成文件的大小。


#![allow(unused)]
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

// impl Summary for Post {
//     // fn summarize(&self) -> String {
//     //     format!("文章{}, 作者是{}", self.title, self.author)
//     // }
// }

pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    // fn summarize(&self) -> String {
    //     format!("{}发表了微博{}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    // println!("{}",post.summarize());
    println!("1 new weibo: {}", weibo.summarize());
}






