/*
    Lifetimes
    - Dangling Reference
    - Undetermined Lifetime
    - Generic Lifetime Parameters
    - GLP typical needed with outputs from functions that are references
    - Issue with GLP
    - GLP with multiple variables
    - GLP and structures
    - Reference to the same variable
*/
// fn main() {
//     let i: &i32;
//     {
//         let j = 5;
//         i = &j;
//     }
//     println!("{}", i)
// }

// fn main() {
//     let some_int = 10;
//     let additional_int = some_fn(&some_int);
//     println!("{}", additional_int);
// }

// fn some_fn(i: &i32) -> &i32 {
//     &i
// }

// fn main() {
//     let int1 = 5;
//     let int2 = 10;
//     let result = greater(&int1, int2);
// }

// fn greater<'a>(i: &'a i32, j: i32) -> &'a i32 {
//     i
// }

// fn main() {
//     let int1 = 11;
//     {
//         let int2 = 10;
//         let result = greater(&int1, &int2);
//         println!("The larger value is {}", result);
//     }
// }

// fn greater<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
//     if i > j {
//         i
//     } else {
//         j
//     }
// }

// fn main() {
//     let s_1 = "Hello";
//     let v;
//     {
//         let s_2 = String::from("World");
//         v = some_fn(s_1, s_2.as_str());
//     }
//     println!("{}", v);
// }

// fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'b str {
//     second_str
// }

// struct Person<'a> {
//     name: &'a str,
//     age: i32,
// }

// fn main() {
//     let first_name = "Pasindu";
//     let mut pasindu = Person {
//         name: &first_name,
//         age: 25,
//     };

//     {
//         let last_name = String::from("Akalpa");
//         pasindu.name = &last_name;
//     }

//     // println!(
//     //     "The name of the person is {} and his age is {}",
//     //     pasindu.name, pasindu.age
//     // );
// }

fn main() {
    let some_vec = vec![5, 8, 9, 8, 7, 5, 2];
    let return_vec = use_vec(&some_vec, &some_vec);
}

fn use_vec<'a>(vec1: &'a [i32], vec2: &'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        vec1
    } else {
        vec2
    }
}
