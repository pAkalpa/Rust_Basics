use std::io::stdin;

fn main() {
    /*
    Functions
        - basic function (no inputs and outputs)
        - functions with parameters
        - functions with return type
        - functions with multiple return
    */

    basic_fn();
    function_with_inputs("Pasindu Akalpa", 25);

    let name = "Nevon";
    let age = 9;

    function_with_inputs(name, age);

    println!(
        "Value of the functions_with_return(4, 5) = {}",
        function_with_return(4, 5)
    );

    let (mul, add, sub) = function_with_multiple_return(3, 4);
    println!(
        "Multiplication = {}, Addition = {}, Substraction = {}",
        mul, add, sub
    );

    let full_name = {
        let first_name = "Pasindu";
        let last_name = "Akalpa";
        format!("{} {}", first_name, last_name)
    };

    println!("My full name is {}", full_name);

    /*
    Inputs from user
    */

    let mut n = String::new();
    stdin().read_line(&mut n).expect("failed to read input.");

    let n: f64 = n.trim().parse().expect("Invalid input");
    println!("{:?}", n);
}

fn basic_fn() {
    println!("This is a basic function");
}

fn function_with_inputs(name: &str, age: i32) {
    println!("The name is {} and age is {}", name, age);
}

fn function_with_return(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn function_with_multiple_return(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
