#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    // etc.
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn main() {
    // if let Cuts Out match Boilerplate
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {max}"),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // Using if let else To Recreate The _ match Arm
    let coin = Coin::Penny;
    let mut _count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //     _ => _count += 1
    // }
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        _count += 1;
    }
}