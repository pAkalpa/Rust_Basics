/*
   Result Enum
   - Basic syntax and usage
*/

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn divition(divident: f64, divisor: f64) -> Result<f64, String> {
//     //  if divisor == 0.0 {
//     //      Err(String::from("Error: Division by zero"))
//     //  } else {
//     //      Ok(divident / divisor)
//     //  }
//     match divisor {
//         0.0 => Err(String::from("Error: Divison by zero")),
//         _ => Ok(divident / divisor),
//     }
// }

// fn main() {
//     println!("{:?}", divition(9.0, 3.0));
//     println!("{:?}", divition(4.0, 0.0));
//     println!("{:?}", divition(0.0, 2.0));
// }

fn main() {
    let some_vec = vec![5, 5, 2, 1, 5, 9];
    let result = match some_vec.get(6) {
        Some(a) => Ok(a),
        None => Err("The value does not exist"),
    };

    println!("The value of result is {:?}", result);
}
