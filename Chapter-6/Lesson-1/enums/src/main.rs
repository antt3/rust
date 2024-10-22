fn main() {
    // The Standard Library Actually Has A Defenition For This Already...
    enum IPAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let _home = IPAddr::V4(127, 0,0, 1);

    let _loopback = IPAddr::V6(String::from("::1"));

    // Different Variants Within 1 Enum
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Impl Is The Same As Structs
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T> > null
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
}