mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// When bringing in functions it's idiomatic to specify up to
// the parent module, to specify the function isn't locally defined
pub use crate::front_of_house::hosting;

mod customer {
    // Other solution when adding customer mod, instead of adding super::
    // use crate::front_of_house::hosting;

    pub fn _eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}