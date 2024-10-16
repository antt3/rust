fn main() {
    // Reference Basics
    fn _ex_fn_1() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }

    fn calculate_length(s: &String) -> usize {
        // s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership
      // of what it refers to, it is not dropped.

    // Mutable References
    fn _ex_fn_2() {
        let mut s = String::from("hello");

        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // References Using Scope To Prevent Data Races
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, we can make a new ref with no problems

    let r2 = &mut s;

    // Using Mutable & Immutable Refs In The Same Scope
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 & r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}
