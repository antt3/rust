fn main() {
    // Empty vectors must have an explicit type
    let _v: Vec<i32> = Vec::new();

    // The vector macro
    let _v: Vec<i32> = vec![1, 2, 3];

    // The vectors type is inferred by its later use
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Referencing an index of a vector
    let v = vec![1, 2, 3, 4, 5];

    // Will panic if index is out of bounds
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Handles out of bounds error
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Looping through a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Storing multiple types using an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    let mut v = vec![1, 2, 3, 4, 5];

    // Testing the pop method
    let five = v.pop();
    match five {
        Some(five) => println!("Popped the last element: {five}"),
        None => println!("The vector is empty, nothing was popped."),
    }
}
