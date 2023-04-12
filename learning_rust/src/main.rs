/*
    Enums
        - General Syntax
        - Enums with attached data
        - Enums to create vector with different types pf data
*/

// enum Conveyance {
//     Car(i32),
//     Train(i32),
//     Air(i32),
// }

// impl Conveyance {
//     fn travel_allowance(&self) -> f32 {
//         let allowance = match self {
//             Conveyance::Car(miles) => *miles as f32 * 14.0 * 2.0,
//             Conveyance::Train(miles) => *miles as f32 * 18.0 * 2.0,
//             Conveyance::Air(miles) => *miles as f32 * 30.0 * 2.0,
//         };
//         allowance
//     }
// }

// fn main() {
//     let participant_1 = Conveyance::Car(60);
//     // println!("The value of the option is {}", participant_1 as i32);

//     let participant_2 = Conveyance::Air(120);
//     let participant_3 = Conveyance::Train(60);

//     println!(
//         "The participant 1 has a travel allowance of {}",
//         participant_1.travel_allowance()
//     );
//     println!(
//         "The participant 2 has a travel allowance of {}",
//         participant_2.travel_allowance()
//     );
//     println!(
//         "The participant 3 has a travel allowance of {}",
//         participant_3.travel_allowance()
//     );
// }

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}

fn main() {
    let some_value = vec![Value::Integer(12), Value::Float(15.5)];
    println!(
        "The value of the integer is {:?} and the value of the float is {:?}",
        some_value[0], some_value[1]
    );

    for i in some_value.iter() {
        match i {
            Value::Integer(num) => println!("The value is an integer with a value of {}", num),
            Value::Float(num) => println!("The value is a float with a value of {}", num),
        }
    }
}
