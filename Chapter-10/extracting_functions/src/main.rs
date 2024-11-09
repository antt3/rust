// Abstracted the repeated code to find the largest number in a list into its own function
fn largest_num(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // Finding the largest number in a list
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    // A tedious repetition of finding the largest number in a list
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    // Finding the largest number in a list
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_num(&number_list);
    println!("The largest number is {result}");

    // Finding the largest number in a list again with less repetitive code
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_num(&number_list);
    println!("The largest number is {result}");
}
