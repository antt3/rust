fn main() {
    // Slices Without Slices
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has a value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // String Slices
    let s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..];

    // first_word With Slices
    fn first_word_2(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // String Literals As Slices
    let my_string = String::from("hello world");
    
    // `first_word_2` works on slices of `String`s, whether partial or whole
    let word = first_word_2(&my_string[0..6]);
    let word = first_word_2(&my_string[..]);
    // `first_word_2` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word_2(&my_string);

    let my_string_literal = "hello world";

    // `first_word_2` works on slices of string literals, whether partial or whole
    let word = first_word_2(&my_string_literal[0..6]);
    let word = first_word_2(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_2(my_string_literal);

    // Array Slices
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}