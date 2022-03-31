fn add_i8(a:i8, b:i8) -> i8 {
    a + b
}
fn add_i32(a:i32, b:i32) -> i32 {
    a + b
}
fn add_f64(a:f64, b:f64) -> f64 {
    a + b
}

fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

fn largest<T: std::cmp::PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T :std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy>  Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn sqrt_distance(&self) -> T {
        self.x*self.x + self.y*self.y
    }

    fn sqrt_distance_2(&self, other: &Self) -> T {
        (self.x - other.x)*(self.x - other.x) + (self.y - other.y)*(self.y - other.y) 
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main1() {
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i32: {}", add_i32(20, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));

    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    let number_list = vec![34,50,24,100,65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer_p1 = Point {x: 3, y: 4};
    let integer_p2 = Point {x: 6, y: 8};
    let float = Point {x: 3.0, y: 4.0};
    
    println!("{} {:.2}", integer_p1.x(),float.x());

    println!("{:#?} {:#?}",integer_p1,float);
    println!("{}", float.distance_from_origin());
    println!("{}", float.sqrt_distance().sqrt());
    println!("{}", (integer_p1.sqrt_distance() as f64).sqrt());

    println!("{}", ((integer_p1.sqrt_distance_2(&integer_p2)) as f64).sqrt());
}

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

