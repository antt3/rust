// When bringing in structs, enums, & other items (Not Functions)
// it's idiomatic to specify the full path
use std::collections::HashMap;
use rand::Rng;

// If bringing 2 items with the same name into scope
// then the parent modules will need to be specified as well
use std::fmt;
use std::io;

// Or rename 1 to resolve the naming conflict
use std::fmt::Result;
use std::io::Result as IOResult;

// Nested Path Ex 1
// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};
//
// Nested Path Ex 2
// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// The Glob Operator
use std::collections::*;

fn _function1() -> fmt::Result {
    todo!();
}

fn _function2() -> io::Result<()> {
    todo!();
}

fn _function3() -> Result {
    todo!();
}

fn _function4() -> IOResult<()> {
    todo!();
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let _secret_number = rand::thread_rng().gen_range(1..100);
}