fn main() {
    // There are multiple ways to create a String
    let mut _s = String::new();

    let data = "initial contents";

    let _s = data.to_string();

    // the method also works on a literal directly
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    // UTF-8 encoded String examples
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שלום");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // There are many ways to append to a String
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str takes a string slice to prevent ownership of its parameter
    s1.push_str(s2);
    println!("s2 is {s2}");

    // Push appends a single character
    let mut s = String::from("lo");
    s.push('l');

    // The + operator is a funtion called add
    // The function will take ownership of the first string,
    // but requires a reference for subsequent strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let _s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

    // Use the format macro for complicated strings
    // It uses references so it doesn't take ownership of any parameters
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{s1}-{s2}-{s3}");

    // Strings can be seen as bytes, scalar values, & grapheme clusters
    // The String
    let _ex = String::from("नमस्ते");
    // The String in bytes
    let _ex_bytes: Vec<[u8; 18]> = vec![[
        224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    ]];
    // The String in scalar values
    let _ex_scala: Vec<[char; 6]> = vec![['न', 'म', 'स', '्', 'त', 'े']];
    // The String in grapheme clusters
    let _ex_grapheme: Vec<[&str; 4]> = vec![["न", "म", "स्", "ते"]];

    // String sclices can panic and use bytes
    let hello = "Здравствуйте";

    let _s = &hello[0..4]; // returns "Зд" which is not clear visually

    // Seperates & returns chars
    for c in "Зд".chars() {
        println!("{c}");
    }

    // Seperates & returns bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Note: Valid Unicode scalar values may be made up of more than one byte.
    // Testing the contains & replace methods
    let _t_or_f = String::from("Hello, World!").contains("W");
    println!("{_t_or_f}");

    let mut s = String::from("foobat");
    s = s.replace("bat", "bar");
    println!("{s}");
}
