pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}

// cfg means Configuration, and is what prevents the
// test code from compiling when not running cargo test
#[cfg(test)]
mod tests {
    use super::*;

    // The below tests are all Unit Tests, testing the file they're in
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Testing private functions is allowed in Rust
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}
