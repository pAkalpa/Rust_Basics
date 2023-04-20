/*
    Closures
    - Basic Syntax
    - Closure with inputs
    - Same variable for different closure
    - Ownership rules and closures
    - Inference of the output
    - Passing a closure as a function argument
    - Borrow by immutable reference
    - Moving of a value into a closure
*/

// |...| {...}

// fn main() {
//     let x = 5;
//     let square = |num: i32| println!("The square of {} is {}", num, num * num);
//     let square = |num: i32| println!("The cube of {} is {}", num, num * num * num);
//     square(x);

//     let y = 15;
//     square(y);
// }

// fn main() {
//     let print_user_age = |general_info: String, name: &str, age: i32| {
//         println!("{} \n\t {} \n\t {}", general_info, name, age)
//     };

//     let general_info = String::from("The details are");
//     let (person_name, person_age) = (String::from("Pasindu"), 51);

//     print_user_age(general_info, &person_name, person_age);
//     println!("The variable has been moved {}", person_name);
// }

// fn main() {
//     let square = |num| num * num;
//     // let x = 5;
//     // square(x);

//     let y = 5.5;
//     square(y);
// }

// fn main() {
//     let division_status = |y: f32| {
//         if y != 0.0 {
//             true
//         } else {
//             false
//         }
//     };
//     division(5.0, 10.0, division_status);
//     division(55.0, 0.0, division_status);
// }

// fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
//     if f(y) {
//         println!("The division result is {}", x / y);
//     } else {
//         println!("Division is not possible");
//     }
// }

fn main() {
    let mut vec_1 = vec![1, 2, 3];
    let mut some_closure = || {
        let vec_2 = vec_1;
    };

    // println!("vec 1 {:?}", vec_1);

    some_closure();
    // println!("vec 1 = {:?}", vec_1);
    // println!("vec 2 = {:?}", vec_2);
}
