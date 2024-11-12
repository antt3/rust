#![allow(unused)]
use std::fmt::Display;

// Ye Olde Lifetime Elision Rules
// 1. The compiler assigns a lifetime parameter to each parameter that's a refernce
fn foo<'a>(x: &'a i32) {}
fn foo2<'a, 'b>(x: &'a i32, y: &'b i32) {}

// 2. If there is exactly one input lifetime parameter,
//    that lifetime is assigned to all output lifetime parameters
fn foo3<'a>(x: &'a i32) -> &'a i32 {x}

// 3. If there are multiple input lifetime parameters,
//    but one of them is &self or &mut self  because this is a method,
//    the lifetime of self is assigned to all output lifetime parameters.
impl <'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b, 'c>(&'b self, announcement: &'c str) -> &'b str {
        println!("Attention please: {announcement}");
        self.part
    }
}

// The func takes 2 params which live at least as long as the lifetime 'a
// The returned val will live at least as long as lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// To define a struct to hold references it needs a lifetime annotation
// on every reference in the struct's definition
struct ImportantExcerpt<'a> {
    part: &'a str
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Lifetimes are a type of generic, the declarations of the lifetime
// parameter 'a and the generic type parameter T go in the same list
// inside the angle brackets after the function name
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where 
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = 5;

    let r = &x;

    println!("r: {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // &i32         // a reference
    // &'a i32      // a reference with an explicit lifetime
    // &'a mut i32  // a mutable reference with an explicit type

    let string1 = String::from("long string is long!");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    } // result is no longer valid because string2 is out of scope

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetimes can live for the duration of the program
    // All string literals have the 'static lifetime
    // because they are stored in the program's binary, which is always available
    let s: &'static str = "I have a static lifetime.";
    // Usually error messages suggesting to use 'static are from attempting
    // to create dangling references or a mismatch of the available lifetimes
    // In such cases, the solution is to fix the problems, not to specify 'static

}