#![allow(unused)]
fn main() {
    use std::collections::HashMap;

    // new Creates an Empty Hashmap
    // Type should be defined, if not defined implicitly later
    let mut scores = HashMap::new();

    // insert adds elements
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get to get an element
    // get returns Option<&V> use copied to get an Option<V>
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Will print each pair in arbitrary order
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();

    // insert overwrites the value if there is one
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // entry returns an enum called Entry
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert returns a mutable reference &mut V
        let count = map.entry(word).or_insert(0);
        // it must be dereferenced with * to assign it
        *count += 1;
    } // The mutable reference goes out of scope, borrow checker is happy

    println!("{map:?}");

}