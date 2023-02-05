use std::mem::size_of_val;

fn main() {
    /*
    Tuples
        - destructuring tuple
        - nested tuples
    */

    let my_information = ("Pasindu Akalpa", 25);
    println!(
        "Name is {} and age is {}",
        my_information.0, my_information.1
    );

    let (name, age) = my_information;
    println!("{} -> {}", name, age);

    let nested_tuple = (4, 5.0, (3, 2), "Hello");
    let element = nested_tuple.2 .0;
    println!("The value of element is {}", element);

    let empty_tuple = ();

    /*
    Arrays
        - update elements
        - string and char arrays
        - functions on array
    */

    let mut number_array: [i32; 5] = [4, 5, 6, 7, 8];
    println!("{}", number_array[0]);

    println!("{:?}", number_array);

    number_array[4] = 5;
    println!("{:?}", number_array);

    let array_with_same_elements = [0; 10];

    let mut string_array_1 = ["Apple", "Tomato", "Grapes"];
    let string_array_2 = ["Unknown"; 6];

    string_array_1[0] = "Carrot";

    let char_array = ['a', 'p', 'p', 'l', 'e'];

    let mut number_array_1: [i32; 5] = [4, 5, 6, 7, 8];
    let subset_array = &number_array_1[0..=3];

    println!(
        "The subset of the values of the array are {:?}",
        subset_array
    );

    println!("Elements in the array are {}", number_array_1.len());

    println!(
        "The array is occupying {} bytes",
        size_of_val(&number_array_1)
    );

    // number_array_1[10] = 5;

    let check_index = number_array_1.get(2);
    println!("{:?}", check_index);
}
