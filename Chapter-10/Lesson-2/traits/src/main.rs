#![allow(unused)]
use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;

    // fn summarize_author(&self) -> String;
    
    // fn summarize(&self) -> String {
    //     format!("(Read more from {}...)", self.summarize_author())
    // }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    // fn summarize_author(&self) -> String {
    //     format!("@{}", self.username)
    // }
}

// Will only accept structs that implement Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// &impl Trait is syntactic sugar for a trait bound
pub fn notify_long_form<T: Summary>(item: &T) {}

// Both parameters must implement Summary
pub fn notify_w_two_params(item1: &impl Summary, item2: &impl Summary) {}

// Uses the long form to specify both parameters must also be the same type
pub fn notify_same_types<T: Summary>(item1: &T, item2: &T) {}

// item must implement both Display and Summary
pub fn notify_multiple_trait_bounds(item: &(impl Summary + Display)) {}

// + is also valid with trait bounds on generic types
pub fn notify_mult_trait_bounds_on_generic<T: Summary + Display>(item: &T) {}

// Multiple generic parameters with multiple trait bounds can become hard to read
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> () {}

// Using where helps the function signature look less cluttered
fn some_func_using_where<T, U>(t: &T, u: &U) -> ()
where
    T: Display + Clone,
    U: Clone + Debug
{}

// A trait can also be used as a return type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// This implementation is for any pair struct type
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This impl is only for pair structs with a type implementing Display & PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// This blanket implementation implements the ToString method
// on any type that implements the Display trait
// impl<T: Display> ToString for T {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    println!("1 new tweet: {}", tweet.summarize());

}