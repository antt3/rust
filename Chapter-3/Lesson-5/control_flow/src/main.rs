fn main() {
    // If/Else In A Let Statement
    let condition = true;
    let number = if condition { 3 } else { 6 };

    // If/Else If/Else Statements
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("mumber is not divisible by 4, 3, or 2");
    }

    // For Loops
    loop {
        println!("again!");
        break;
    }

    // For Loop Returning A Value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Loop Labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // While Loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // For Loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {element}");
    }

    // Refactored While Loop Using For
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
