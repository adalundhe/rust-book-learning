#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    // Match operators map values of an enum
    // to code blocks (methods attached to the
    // enum option) that executes if matched.
    // We can run multiple lines of code in a match
    // function block by using curly brackets.

    // Note that we can bind match "arms" to the values
    // that match the pattern, thus extracting the
    // value out of an enum option/variant.
    match coin {
        Coin::Penny => {
            1
        },
        Coin::Nickle => {
            5
        },
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from - {:?}.", state);
            25
        },
    }
}

fn main() {

    let penny = Coin::Penny;
    let nickle = Coin::Nickle;
    let dime = Coin::Dime;
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    let alaska_quarter = Coin::Quarter(UsState::Alaska);

    let mut cents = value_in_cents(penny);
    println!("Got: {} cents!", cents);

    cents = value_in_cents(nickle);
    println!("Got: {} cents!", cents);

    cents = value_in_cents(dime);
    println!("Got: {} cents!", cents);

    cents = value_in_cents(alabama_quarter);
    println!("Got: {} cents!", cents);

    cents = value_in_cents(alaska_quarter);
    println!("Got: {} cents!", cents);
    
}
