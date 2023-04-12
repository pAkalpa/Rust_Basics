/*
   Option Enum
   - Basic Syntax
*/

// enum Option<T> {
//     None,
//     Some(T),
// }

// fn main() {
//     let mut disease: Option<String> = None;
//     disease = Some(String::from("Diabetes"));

//     match disease {
//         Some(disease_name) => println!("You have the disease of {}", disease_name),
//         None => println!("You do not have any disease"),
//     }
// }

// fn main() {
//     let s1: Option<&str> = Some("Some String");
//     println!(
//         "The value of s1 is {:?}\n the value of s1 iteself is {:?}",
//         s1,
//         s1.unwrap()
//     );

//     let f1: Option<f64> = Some(10.54);
//     let mut f2 = 16.5;

//     f2 = f2 + f1.unwrap();

//     println!("The value of f2 is {}", f2);

//     let v1: Option<Vec<i32>> = Some(vec![0, 1, 2, 3]);
//     let p1 = Person {
//         name: String::from("Pasindu"),
//         age: 25,
//     };

//     let some_one: Option<Person> = Some(p1);
// }

// struct Person {
//     name: String,
//     age: i32,
// }

fn square(num: Option<i32>) -> Option<i32> {
    match num {
        Some(number) => Some(number * number),
        None => None,
    }
}

fn main() {
    let number = Some(6);

    if square(number) != None {
        println!("The Value of number is {:?}", square(number));
    } else {
        println!("We do not have any value");
    }
}
