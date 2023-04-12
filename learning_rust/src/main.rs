/*
   Hash Maps
   - Defining Hashmap
   - Inserting Values
   - Accessing Values
   - Checking if the value exist
   - Iterating through elements of hashmap
   - Updating a value at a specific key
*/

use std::collections::HashMap;

// fn main() {
//     let mut person: HashMap<&str, i32> = HashMap::new();
//     person.insert("Pasindu", 25);
//     person.insert("Jhon", 40);
//     person.insert("Jane", 30);

//     println!("The age is {:?}", person.get("Jhon").unwrap());

//     if person.contains_key("Pasindu") {
//         println!("The value exist");
//     } else {
//         println!("The value does not exist");
//     }

//     match person.get("Pasindu") {
//         Some(Value) => println!("The value exists {}", Value),
//         None => println!("The value does not exists"),
//     }

//     for (name, age) in &person {
//         println!("The person {} has an age of {}", name, age);
//     }
// }

// fn main() {
//     let mut likes: HashMap<&str, &str> = HashMap::new();
//     //  likes.insert("Pasindu", "games");
//     //  likes.insert("Pasindu", "apple");

//     //  println!("The things which is being liked is {:?}", likes)
//     likes.entry("Pasindu").or_insert("apple");
//     likes.entry("Pasindu").or_insert("asus");

//     println!("The things which is being liked is {:?}", likes)
// }

fn main() {
    let some_vec: Vec<i32> = vec![5, 5, 8, 8, 1, 0, 1, 5, 5, 5, 5];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}", freq_vec);
}
