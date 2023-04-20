/*
   Function Types
   - Basic Syntax and Use
   - Function types as parameter to functions
*/
// fn main() {
//     let mut f = min;
//     let mut t = max;
//     println!("The min of the two value is {}", f(2, 3));
//     println!("The max of the two value is {}", t(2, 3));
// }

// fn max(x: i32, y: i32) -> i32 {
//     if x > y {
//         x
//     } else {
//         y
//     }
// }

// fn min(x: i32, y: i32) -> i32 {
//     if x < y {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let (my_name, my_age) = (String::from("Pasindu"), 25);
//     prints_full_info(prints_name, &my_name, my_age);
// }

// fn prints_name(name: &str) {
//     print!("The name is {}", name);
// }

// fn prints_full_info(f: fn(&str), some_name: &str, age: i32) {
//     f(some_name);
//     println!(" and my age is {}", age)
// }

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);
}
