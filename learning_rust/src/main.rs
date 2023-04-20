/*
   Iterators
   - Basics of iterators and its syntax
   - Some useful functions for iterators
   - Common statistic related functions
   - Modifying and collecting values
*/
fn main() {
    // let some_vec = vec![1, 2, 3, 4, 5, 6, 7];
    // let mut iter = some_vec.iter();
    // println!("The iterator : {:?}", iter);
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());
    // println!("{:?}", iter.next());

    // let a: Vec<u32> = vec![0, 1, 2, 4, 5, 6, 9, 8, 7];
    // let mut check = a.iter().any(|&x| x > 0);
    // println!("The value of the any function is {}", check);

    // let check = a.iter().all(|&x| x >= 0);
    // println!("The value of all function is {}", check);

    // let check = a.iter().find(|&&x| x > 0);
    // println!("The value of function is {}", check.unwrap());

    // let check = a.iter().position(|&x| x > 4);
    // println!("The value of function position is {}", check.unwrap());

    // let check = a.iter().rposition(|&x| x > 4);
    // println!("The value of function rposition is {}", check.unwrap());

    // let check = a.iter().max();
    // println!("The value of function max is {}", check.unwrap());

    // let check = a.iter().min();
    // println!("The value of function min is {}", check.unwrap());

    // let mut iter = a.iter().rev();
    // println!("The result of applying rev function : {:?}", iter.next());

    let a = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let filtered_values = a.iter().filter(|&x| *x >= 5).collect::<Vec<&u32>>();
    println!("{:?}", filtered_values);

    let b = a.clone();
    let filtered_values = a.into_iter().filter(|x| *x >= 5).collect::<Vec<u32>>();
    println!("{:?}", filtered_values);

    let mapped_values = b.iter().map(|x| 2 * *x).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);

    let mapped_values = b
        .iter()
        .map(|x| 2 * *x)
        .filter(|x| *x > 10)
        .collect::<Vec<u32>>();
    println!("{:?}", mapped_values);
}
