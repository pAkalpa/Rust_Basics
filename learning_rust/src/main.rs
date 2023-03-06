use std::io::stdin;

fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    return vec;
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped_value = stack.pop();
    println!("poped value is {:?}", poped_value);
    return poped_value;
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Cannot add more elements")
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size(stack: &Vec<u32>) -> usize {
    return stack.len();
}

fn input() -> u32 {
    let mut n = String::new();
    stdin().read_line(&mut n).expect("Failed to read input");
    let n: u32 = n.trim().parse().expect("Invalid input");
    return n;
}

fn main() {
    /*
    Stack
       - Stack using vec
    */

    println!("Lets first create a stack for our use");
    println!("Please mention the size of the stack");

    let size_stack = input();
    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("\n\n ***** Menu ***** \n");
        println!("1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit");
        println!("\n Enter your choice: ");

        let choice = input();
        match choice {
            1 => {
                println!("Enter the value to be inserted: ");
                let item = input();
                push(&mut stack, item, size_stack as usize);
            }
            2 => println!("The element which is poped is {:?}", pop(&mut stack)),
            3 => println!("The elements are {:?}", stack),
            4 => println!("The size of the stack is {}", size(&stack)),
            5 => break,
            _ => println!("\n Wrong Selection !!! Try Again !!!"),
        }

        println!("Do you want to continue 1 = Yes / 0 = No");
        let status = input();
        if status == 1 {
            continue;
        } else {
            break;
        }
    }
}
