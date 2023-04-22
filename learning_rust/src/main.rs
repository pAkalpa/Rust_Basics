/*
   Modules
*/
// fn main() {
//     let rect1 = Rectangle {
//         length: 5,
//         width: 10,
//     };

//     rect_area(&rect1.length, &rect1.width);
// }

// struct Rectangle {
//     length: i32,
//     width: i32,
// }

// fn some_fn() {
//     println!("This is the function the main crate");
// }

// mod maths {
//     pub mod basic_math {
//         pub fn multiplication(num1: &i32, num2: &i32) -> i32 {
//             let result = num1 * num2;
//             printing(&result);
//             result
//         }

//         fn printing(num: &i32) {
//             println!("The result is {}", num);
//             crate::some_fn();
//         }
//     }
// }

// pub fn rect_area(length: &i32, width: &i32) -> i32 {
//     use maths::basic_math::multiplication;
//     multiplication(length, width)
// }

// mod file_2;

// fn main() {
//     file_2::some_person();
// }

mod file_3;

fn main() {
    file_3::allowance();
}
