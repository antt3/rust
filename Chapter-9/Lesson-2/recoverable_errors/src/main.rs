// The Result enum definition
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
#![allow(unused)]
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// Box<dyn Error> is a trait object
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    // match is a primitive, it's not very concise

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // unwrap will return the Ok() value or panic if there's an error
    let greeting_file = File::open("hello.txt").unwrap();

    // expect works the same as unwrap but with a custom error message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // io::Error is the type of error returned from open & read_to_string
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        
        // implicitly returns the result of the match
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // ? returns the value in an Ok function
    // ? returns the Error to the calling code like match in the last func
    // ? converts the error using from to the defined error type of the func
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // Shortened from the last func using ? & chained method calls
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // Shortened again using the standard library 
    fn read_username_from_file_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // ? also works with Option<T>
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    let greeting_file = File::open("hello.txt")?;

    Ok(())
}