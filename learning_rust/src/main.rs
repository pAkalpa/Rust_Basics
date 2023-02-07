fn main() {
    /*
    Ownership and functions
    */

    let stack_num = 32;
    let mut heap_num = vec![4, 5, 6];
    stack_function(stack_num);
    println!(
        "The stack variable is copied and the original value was {}",
        stack_num
    );
    heap_function(&mut heap_num);
    println!("The value of the vector inside function is {:?}", heap_num);
}

fn stack_function(mut var: i32) {
    var = 56;
    println!(
        "The copied value of the variable has been updated to {}",
        var
    );
}

fn heap_function(var: &mut Vec<i32>) {
    var.push(7);
    println!("The value of the vector inside function is {:?}", var);
}
