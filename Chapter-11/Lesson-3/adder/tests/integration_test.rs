// This is a seperate crate from adder so it needs to be brought into scope
use adder::add_two;

mod common;

// Don't need to add #[cfg(test)] because a "tests" dir is treated differently by Cargo
#[test]
fn it_adds_two() {
    common::setup();

    let result = add_two(2);
    assert_eq!(result, 4);
}