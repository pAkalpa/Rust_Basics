fn main() {
    // single line comment

    /*
    Multiline Comment
     */

    println!("This is println!()");
    print!("This is print!()");

    println!(
        "\nThis is how to print non string values {} \nprintln!(\"{{}} \", value)",
        10
    );

    println!(
        "My First Name is {}, my last name is {}",
        "Pasindu", "Akalpa"
    );

    println!(
        "This
    is Multiline
    Print Statement"
    );

    println!("Multiple indexed Placeholders {2} -> {1} -> {0}", 1, 2, 3);

    println!("This is {argument}", argument = "named argument");

    println!("The sum of 10 + 20 = {}", 10 + 20);
}
