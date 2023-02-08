fn main() {
    /*
    References Rules
        - One mutable reference in a scope
        - Many immutable references
        - mutable and immutable cannot coexist
        - scope of a reference
        - data should not change when immutable references are in scope
    */

    // let mut heap_num = vec![4, 5, 6];
    // let ref1 = &mut heap_num;
    // let ref2 = &mut heap_num;
    // println!(
    //     "The First reference is {:?} and second reference is {:?}",
    //     ref1, ref2
    // );

    // let mut heap_num = vec![4, 5, 6];
    // let ref1 = &heap_num;
    // let ref2 = &heap_num;
    // println!(
    //     "The First reference is {:?} and second reference is {:?}",
    //     ref1, ref2
    // );

    // let mut heap_num = vec![4, 5, 6];
    // let ref1 = &heap_num;
    // let ref2 = &heap_num;
    // let ref3 = &mut heap_num;
    // println!(
    //     "The First reference is {:?} and second reference is {:?} and third reference is {:?}",
    //     ref1, ref2, ref3
    // );

    // let mut heap_num = vec![4, 5, 6];
    // let ref1 = &heap_num;
    // let ref2 = &heap_num;
    // println!("Immutable references are {:?} and {:?}", ref1, ref2);

    // let ref3 = &mut heap_num;
    // ref3.push(7);
    // println!("The First reference is {:?}", ref3);

    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("Immutable references are {:?} and {:?}", ref1, ref2);
    heap_num.push(86);
}
