fn main() {
    /*
    Rust Ownership
        - each value in rust has a variable that's called its owner.
        - There can be only one owner at a time.
        - when the owner goes out of scope, the value will be dropped.
    */

    // copy and move
    // primitives and non-primitives
    // move = leads to change of ownership
    // reference = leads to concept of borrowing

    /*
    let mut x = 32;
    let mut y = x;
    println!("The value of x = {} and the value of y = {}", x, y);

    let s1 = String::from("abc");
    let s2 = &s1; // borrowing and referencing
    println!("The value of s1 = {} and the value of s2 = {}", s1, s2);
     */

    let num_vec1: Vec<i32> = vec![5, 6, 7, 8, 9];
    let num_vec2 = &num_vec1;
    println!(
        "The first vector is {:?} and second vector is {:?}",
        num_vec1, num_vec2
    );

    let num_vec2 = num_vec1.clone();
    println!(
        "The first vector is {:?} and second vector is {:?}",
        num_vec1, num_vec2
    );

    {
        let my_name = String::from("pasindu Akalpa");
    }

    // println!("My name is {}", my_name);
}
