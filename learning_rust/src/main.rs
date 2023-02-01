fn main() {
    /*
    Variables in Rust
    */

    let mut x: i64 = 15;
    println!("The value of the variable x = {}", x);

    x = 60;

    let y: i32 = 5 * 5;

    /*
    Data Types
           - Scalar Data Types
               - integer
                   - Signed = i8 i16 i32 i64
                       2^(i - 1) -1 to 2^(i - 1)-1

                   - Unsigned
    */

    println!("The maximum number in i8 is equal to {}", std::i8::MAX);
    println!("The maximum number in u8 equals to {}", std::u8::MAX);

    /*
    Floats
      - f32 f64
    */

    let z: f64 = 3.6;

    println!("The maximum number in f32 is {}", std::f32::MAX);

    /*
    Boolean
    */

    let status: bool = false;
    println!(
        "the values of the some of our variables are {:?}",
        (x, y, z, status)
    );

    let not_equals: bool = 18 != 18;
    println!("The value of condition 18 != 18 is {}", not_equals);

    /*
    Charaters
    */

    let c1: char = 'a';
    let c2: char = '3';
    let c3: char = '+';
    let c4: char = '\u{288A}';
    let c5: char = '\"';

    println!(
        "The values of defined chars are c1 = {}, c2 = {}, c3 = {}, c4 = {}, c5 = {}",
        c1, c2, c3, c4, c5
    );
}
