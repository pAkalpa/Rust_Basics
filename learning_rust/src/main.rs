fn main() {
    /*
    For loops
        - Simple for loop
    */

    // let mut some_vec = vec![45, 30, 85, 90, 41, 39];
    // for i in some_vec.iter() {
    //     println!("{}", i);
    // }

    // println!("{:?}", some_vec);

    let mut some_vec = vec![45, 30, 85, 90, 41, 39];

    for i in &mut some_vec {
        *i += 5;
    }

    println!("{:?}", some_vec);
}
