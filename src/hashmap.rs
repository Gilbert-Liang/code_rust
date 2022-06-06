// #![allow(unused)]
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     // println!("第三个元素是 {}", third);

//     // let does_not_exist = &v[100];
//     let does_not_exist = v.get(100);
//     match does_not_exist {
//         Some(x) => println!("100th ele is {}", x),
//         None => println!("100th does not exist"),
//     }

//     match v.get(2) {
//         Some(third) => println!("第三个元素是 {}", third),
//         None => println!("去你的第三个元素，根本没有！"),
//     }
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     let first = &v[0];
//     println!("The first element is: {}", first);
//     v.push(6);
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String)
// }
// fn main() {
//     let v = vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string())
//     ];

//     for ip in v {
//         show_addr(ip)
//     }
// }

// fn show_addr(ip: IpAddr) {
//     println!("{:?}",ip);
// }

// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}", self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}", self.0)
//     }
// }

// fn main() {
//     let v: Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];

//     for ip in v {
//         ip.display();
//     }
// }

// fn main() {
//     use std::collections::HashMap;
    
//     let teams_list = vec![
//         ("中国队".to_string(),100),
//         ("美国队".to_string(),10),
//         ("日本队".to_string(),50),
//     ];

//     // let mut teams_map = HashMap::new();
//     // for team in &teams_list {
//     //     teams_map.insert(&team.0,team.1);
//     // }
//     let teams_map: HashMap<_,_> = teams_list.into_iter().collect();

//     println!("{:?}",teams_map);
// }

fn main() {
    // use std::collections::HashMap;

    //方案1
    // let name = String::from("Sunface");
    // let age = 18;

    // let mut handsome_boys = HashMap::new();
    // handsome_boys.insert(&name, age);

    // // std::mem::drop(name);

    // println!("因为过于无耻，{:?}已经被从帅气男孩名单中除名",name);
    // println!("还有，他的真实年龄远远不止{}岁", age);

    //方案2
    // let mut scores = HashMap::new();
 
    // scores.insert(String::from("Blue"),10);
    // scores.insert(String::from("Yellow"),50);

    // let team_name = String::from("Blue");
    // let score: Option<&i32> = scores.get(&team_name);

    // match score {
    //     Some(x) => println!("{}",x),
    //     None => println!("None"),
    // }

    // let mut scores = HashMap::new();
    // scores.insert("Blue", 10);

    // // 覆盖已有的值
    // let old = scores.insert("Blue", 20);
    
    // println!("old:{:?}",old);
    // assert_eq!(old, Some(10));

    // // 查询新插入的值
    // let new = scores.get("Blue");
    // println!("new: {:?}",new);

    // assert_eq!(new, Some(&20));

    // // 查询Yellow对应的值，若不存在则插入新值
    // let v = scores.entry("Yellow").or_insert(5);
    // println!("value: {} ",*v);
    // assert_eq!(*v, 5); // 不存在，插入5

    // // 查询Yellow对应的值，若不存在则插入新值
    // let v = scores.entry("Yellow").or_insert(50);

    // println!("value: {}",*v);
    // assert_eq!(*v, 5); // 已经存在，因此50没有插入


    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

