/*
   Traits
       - General Explanation
       - Default Implementation
*/

use std::f32::consts::{PI, TAU};

struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
}

trait General_info {
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str;
}

impl General_info for Person {
    fn info(&self) -> (&str, u8, char) {
        (&(self.name), self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.name
    }
}

impl General_info for Student {
    fn info(&self) -> (&str, u8, char) {
        (&(self.name_std), self.age, self.sex)
    }

    fn country_info(&self) -> &str {
        &self.name_std
    }
}

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

struct Square {
    width: f32,
}

trait General_Info {
    fn area(&self) {
        println!("Not Implemented");
    }
    fn perimeter(&self);
}

impl General_Info for Circle {
    // fn area(&self) {
    //     let area_of_circle = PI * (self.radius * self.radius);
    //     println!("The Area of circle is {}", area_of_circle);
    // }

    fn perimeter(&self) {
        let circumference = TAU * self.radius;
        println!("The Circumference of circle is {}", circumference);
    }
}

impl General_Info for Rectangle {
    fn area(&self) {
        let area_of_rectangle = self.width * self.length;
        println!("The Area of rectangle is {}", area_of_rectangle);
    }

    fn perimeter(&self) {
        let perimeter = (self.length * 2.0) + (self.width * 2.0);
        println!("The Perimeter of rectangle is {}", perimeter);
    }
}

impl General_Info for Square {
    fn area(&self) {
        let area_of_square = self.width * self.width;
        println!("The Area of square is {}", area_of_square);
    }

    fn perimeter(&self) {
        let perimeter = 2.0 * (self.width + self.width);
        println!("The Perimeter of square is {}", perimeter);
    }
}

fn main() {
    let person1 = Person {
        name: String::from("Jhon Doe"),
        citizenship: String::from("Unknown"),
        age: 40,
        gender: 'M',
        salary: 40_000,
    };

    let stuent1 = Student {
        name_std: String::from("Jane Doe"),
        age: 15,
        sex: 'F',
    };

    println!("The basic info include {:?}", person1.info());
    println!("The basic info for the student is {:?}", stuent1.info());

    let circle1 = Circle { radius: 12.0 };
    circle1.area();
    circle1.perimeter();

    let rectangle1 = Rectangle {
        width: 12.0,
        length: 24.0,
    };
    rectangle1.area();
    rectangle1.perimeter();

    let square1 = Square { width: 10.0 };
    square1.area();
    square1.perimeter();
}
