#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(USState) => {
            println!("State quarter from {:?}!", USState);
            25
        }
    }
}

fn printer(x: Option<i32>) {
    match x {
        Some(i) => println!("{}", i),
        None => println!("None"),
    }
}

fn main() {
    let coin = Coin::Quarter(USState::Alaska);
    println!("Value in cents: {}", value_in_cents(coin));

    printer(Some(5));
    printer(None);
}
