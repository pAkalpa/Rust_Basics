/*
   Generics
   - Motivation (reducing code duplication)
   - Generics in functions
   - Generics in structs
*/

// fn squarei32(x: i32) -> i32 {
//     x * x
// }

// fn squaref32(x: f32) -> f32 {
//     x * x
// }

// fn main() {
//     println!("The square of the number is {}", squarei32(5));
//     println!("The square of the number is {}", squaref32(5.5));
// }

// fn square<T>(x: T) -> T
// where
//     T: std::ops::Mul<Output = T> + Copy,
// {
//     x * x
// }

// fn add<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
//     x + x
// }

// fn main() {
//     println!("The square of the number is {}", square(5));
//     println!("The square of the number is {}", square(5.5));
//     println!("The add of the number is {}", add(5.5));
// }

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn printing(&self) {
        println!("The value of the points are {:?}, {:?}", self.x, self.y);
    }
}

fn main() {
    let p1 = Point { x: 5, y: 5 };

    let p2 = Point { x: 1.0, y: 4.0 };

    let p3 = Point { x: 5, y: 5.0 };

    p1.printing();
    p2.printing();
    p3.printing();
}
