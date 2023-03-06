fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    return vec;
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let poped_value = stack.pop();
    return poped_value;
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
    } else {
        stack.push(item);
    }
}

fn size(stack: &Vec<char>) -> usize {
    return stack.len();
}

fn main() {
    /*
    Stack
       - Stack using vec
       - Application of stacks (String Reversal)
    */
    let input_string = String::from("Welcome to Rust");
    let size_stack = input_string.len();
    let mut stack = new_stack(size_stack);
    let mut rev_string = String::new();

    for i in input_string.chars() {
        push(&mut stack, i, size_stack);
    }

    for i in 0..size(&stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("The input string is {:?}", input_string);
    println!("The reverse of the string is {:?}", rev_string);
}
