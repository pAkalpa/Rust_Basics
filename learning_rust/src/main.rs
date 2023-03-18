fn main() {
    /*
    Structures
        - Defining a structure
        - Tuple Structure
    */

    let person1 = Person {
        name: String::from("Pasindu Akalpa"),
        citizenship: String::from("Sri Lanka"),
        age: 40,
        gender: 'M',
        salary: 40_000,
    };

    println!(
        "The structure values are {}, {}, {}, {}, {}",
        person1.name, person1.citizenship, person1.age, person1.gender, person1.salary
    );
    println!(
        "The Taxes on person {} is {}",
        person1.name,
        Person::compute_taxes(&person1)
    );

    let person2 = Person::new();
    println!("The Person 2 is initialzed with default values");
    println!(
        "{} , {} , {} , {} , {}",
        person2.name, person2.citizenship, person2.age, person2.gender, person2.salary
    );

    let person3 = Person {
        age: 50,
        name: String::from("Example"),
        ..person1
    };

    println!(
        "The name of the person 3 = {} and his salary is {}",
        person3.name, person3.salary
    );

    let mut person4 = Person::new();
    println!("The default name of the person 4 is {}", person4.name);
    person4.name = String::from("TEST");
    println!("The update name of the person 4 is {}", person4.name);

    let some_nums = Numbers(32, 16);

    println!(
        "The values of the two fields are {}, {}",
        some_nums.0, some_nums.1
    );

    println!("The greater value is {}", some_nums.greater());
    println!("The lesser value is {}", some_nums.lesser());
}

struct Numbers(i32, i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 > self.1 {
            self.0
        } else {
            self.1
        }
    }

    fn lesser(&self) -> i32 {
        if self.0 < self.1 {
            self.0
        } else {
            self.1
        }
    }
}

struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {
    fn new() -> Self {
        Person {
            citizenship: String::from("USA"),
            name: String::from("abc"),
            age: 40,
            gender: 'M',
            salary: 35_000,
        }
    }

    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.) * 0.5
    }
}
