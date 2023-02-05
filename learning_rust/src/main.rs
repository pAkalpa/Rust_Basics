fn main() {
    /*
    Initializing multiple variables
     */

    let (first_number, second_number) = (250, 480.32);
    println!(
        "The first number is {} and the second number is {}",
        first_number, second_number
    );

    /*
    Readability of Large Number
    */

    let large_number = 1_000_000;
    println!("The value of the large number is {}", large_number);

    /*
    Integer overflow
    */

    // let overflow_number: u8 = 256;

    /*
    Decimal numbers in other formats
    */

    let x: i32 = 255;
    println!(
        "The value of variable x in hexadecimal is {:o} and octal is {:X} and in binary {:b}",
        x, x, x
    );

    /*
    Snake case convention
    */

    let Number = 45;

    /*
    Opertaion on number in different formats
    */

    let n1: i32 = 14;
    let n2: f64 = 15.6;
    // let n3 = n1 + n2 as i32;
    let n3 = n1 as f64 + n2;

    println!("{}", n3);

    /*
    Shadowing
    */

    // let s = 5;
    // let s = 5 * 5;
    // println!("The value of the variable s = {}", s);

    let mut s = 5;
    let s = 5 * 5;
    println!("The value of the variable s = {}", s);

    let s = 32;
    println!("The value of variable s = {} is currently a integer", s);

    let s = 'A';
    println!("The variable s = {} is currently a char", s);

    let s = 64.5;
    println!("The variable s = {} is currently a float", s);

    let num = 65;
    {
        let num = 60;
        println!(
            "The value of the variable s inside the inner scope is {}",
            num
        );
    }
    println!("The value of the variable s = {}", num);

    /*
    Constants
    */

    const MAX_SALARY: u32 = 100_000;
    println!("The value of the constant is {}", MAX_SALARY);
}
