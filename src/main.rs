// 填空
// fn main() {
    // //6.1.3
    // let mut s = String::new();
    // s.push_str("hello, world");
    // s.push('!');
 
    // assert_eq!(s, "hello, world!");

    // //6.1.4
    // let mut s =  String::from("hello");
    // s.push(',');
    // s.push_str(" world");
    // s += "!";

    // println!("{}", s);

    // //6.1.5
    // let s = String::from("I like dogs");
    // // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
    // let s1 = s.replace("dogs", "cats");
 
    // assert_eq!(s1, "I like cats");

    //6.1.6
    // let s1 = String::from("hello,");
    // let s2 = String::from("world!");
    // let s3 = s1.clone() + &s2; 
    // assert_eq!(s3,"hello,world!");
    // println!("{}",s1);
//  }

// 6.1.7
// 使用至少两种方法来修复错误
// fn main() {
//     let s =  "hello, world".to_string();
//     greetings(s)
//  }
 
//  fn greetings(s: String) {
//      println!("{}",s)
//  }

// 6.1.8

// 使用两种方法来解决错误，不要新增代码行
// fn main() {
//     let s =  "hello, world".to_string();
//     let s1: &str  = &s;
// }
 
// fn main() {
//     // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
//     // 填空以输出 "I'm writing Rust"
//     let byte_escape = "I'm writing Ru\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//     // 也可以使用 Unicode 形式的转义字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//     println!("Unicode character {} (U+211D) is called {}",
//                 unicode_codepoint, character_name );

//     // 还能使用 \ 来连接多行字符串
//    let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here \
//                          can be escaped too!";
//     println!("{}", long_string);
// }

// 7.1.10
// fn main() {
//     let raw_str = r"Escapes don't work here: \x3F \u{211D}";
//     // 修改以下代码行，让它工作
//     assert_eq!(raw_str, "Escapes don't work here: \\x3F \\u{211D}");

//     // 如果你希望在字符串中使用双引号，可以使用以下形式
//     let quotes = r#"And then I said: "There is no escape!""#;
//     println!("{}", quotes);

//     // 如果希望在字符串中使用 # 号，可以如下使用：
//     let  delimiter = r###"A string with "# in it. And even "##!"###;
//     println!("{}", delimiter);

//     // 填空
//     let long_delimiter = r###"Hello, "##""###;
//     assert_eq!(long_delimiter, "Hello, \"##\"");
//     println!("{}",long_delimiter);
// }


 
 