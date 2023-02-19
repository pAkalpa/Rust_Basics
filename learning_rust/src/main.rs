use std::io::stdin;

fn main() {
    /*
    Loops
        - loops with no condition
        - while loop
    */

    // loop {
    //     println!("This is an infinite loop");
    // }

    // let my_number = 5;

    // let mut guess = false;

    // println!("guess my number which between 1 and 20");

    // while !guess {
    //     let mut input = String::new();
    //     stdin().read_line(&mut input).expect("failed to read input");

    //     let number: u8 = input.trim().parse().expect("Invalid Input");

    //     if my_number == number {
    //         println!("You guessed the number correctly");
    //         guess = true;
    //     } else {
    //         println!("Please try again!");
    //     }
    // }

    println!(
        "Enter a number and i will tell you the next
    number after your number which is divisible by both 2 and 5"
    );

    let mut number = String::new();
    stdin()
        .read_line(&mut number)
        .expect("failed to read input");
    let mut number: u8 = number.trim().parse().expect("Invalid Input");

    let mut divisible_by_2_5 = false;

    while !divisible_by_2_5 {
        number += 1;

        if number % 2 == 0 && number % 5 == 0 {
            println!(
                "The number after your number which is divisible by both 2
            and 5 is {}",
                number
            );
            divisible_by_2_5 = true;
        }
    }
}
