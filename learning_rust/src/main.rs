fn main() {
    /*
    Strings
       - String slices (&str) - Fixed length strings
    */

    let some_string = "Fixed Length String";
    println!("The text inside the string is \"{}\"", some_string);

    /*
    Strings
       - variable length strings
       - adding removing characters
       - operations on strings
       - formatting and concatanating strings
    */

    let mut growable_string = String::from("This string will grow");
    println!("The text inside the string is \"{}\"", growable_string);

    growable_string.push('s');
    println!("Hey the text inside the string is \"{}\"", growable_string);

    growable_string.pop();
    println!("Hey the text inside the string is \"{}\"", growable_string);

    growable_string.push_str(" which i will use");
    println!("Hey the text inside the string is \"{}\"", growable_string);

    println!(
        "I am going to tell you some basic things about the strings, 
    Is the string empty {},
    The length of the string is {},
    The string has {} bytes,
    Does the string contains the word 'use' {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use")
    );

    growable_string.push_str("   ");
    println!(
        "the length of the string before the trim is {},
    length of the string after the trim is {}",
        growable_string.len(),
        growable_string.trim().len()
    );

    let number = 6;
    println!(
        "The value of the number in string is {}",
        number.to_string()
    );

    println!(
        "Is the number really a string {}",
        number.to_string() == "6"
    );

    let some_char = 'a';
    println!("The character in string is {}", some_char.to_string());

    println!(
        "Is the character really a string {}",
        some_char.to_string() == "a"
    );

    let my_name = "Pasindu Akalpa".to_string();
    println!("The string contains my name {}", my_name);

    let empty_string = String::new();
    println!("Length is {}", empty_string.len());

    let s_1 = "Pasindu".to_string();
    let s_2 = "Akalpa".to_string();

    let s_3 = format!("My first name is {} and my last name is {}", s_1, s_2);
    println!("{}", s_3);

    let string_1 = String::from("Pasindu");
    let string_2 = String::from("Akalpa");

    let string_3 = format!("{}{}", string_1, string_2);
    println!("The concatenated string is {}", string_3);
}
