use std::io::stdin;
fn main() {
    /*
    - Condition if
    - Nested if
    */

    println!("Enter a number");
    let mut some_num = String::new();

    stdin()
        .read_line(&mut some_num)
        .expect("failed to read input");

    let some_num: i32 = some_num.trim().parse().expect("invalid input");

    if some_num != 0 {
        if some_num % 2 == 0 {
            println!("The number is even");
        } else {
            println!("The number is odd");
        }
    } else {
        println!("The number is 0 and is neither even or odd");
    }

    let value = if true { 1 } else { 2 };
    println!("value = {} ", value);
}
