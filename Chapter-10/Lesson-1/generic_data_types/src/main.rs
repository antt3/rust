#![allow(unused)]
// Finds the largest number in an i32 list
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Finds the largest character in a char list
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Uses PartialOrd to restrict T to any type that can be ordered
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// struct using generics
// T is 1 type, so x and y must be the same type
struct Point<T> {
    x: T,
    y: T,
}

// T and U can be different types, so x and y can be different types
struct PointTwo<T, U> {
    x: T,
    y: U,
}

// The Option enum utilizing a generic type
enum Option<T> {
    Some(T),
    None,
}

// The Result enum utilizing 2 generic types for a succeed and an error value
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// An implement method using a generic
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// An implement method for one type of the generic struct
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// A struct with 2 seperate generics
struct PointThree<X1, Y1> {
    x: X1,
    y: Y1,
}

// An implement method with generics declared in the impl and the declared method
impl<X1, Y1> PointThree<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointThree<X2, Y2>) -> PointThree<X1, Y2> {
        PointThree {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let both_integer = PointTwo { x: 5, y: 10 };
    let both_float = PointTwo { x: 1.0, y: 4.0 };
    let integer_and_float = PointTwo { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointThree { x: 5, y: 10.4 };
    let p2 = PointThree { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    enum OptionI32 {
        Some(i32),
        None,
    }

    enum OptionF64 {
        Some(f64),
        None,
    }

    // When compiled the generic Option definition is replaced with the above
    // 2 specific enum definitions, Rust performs monomorphization
    let integer = Some(5);
    let float = Some(5.0);
}
