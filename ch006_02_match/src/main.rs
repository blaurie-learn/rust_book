
//rust has a powerful flow control called "match"

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(i32),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {    //use braces for multi-lines
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(year) => {        //Can also easily extract associated values
            println!("Quarter from {}", year);
            25
        },
    }
}


// lets say we only want to do something when we have a value, and it not, do nothing.
// this is rusts concept of nulls
fn plus_one(x: Option<i32>) -> Option<i32> {
    
    // Matches are exhaustive. All variants have to have some behavior attached.
    // You can use the underscore when you don't want to list all possible value

    match x {
        Some(i) => Some (i + 1),
        None => None
    }
}

fn main() {
    println!("value is: {}", value_in_cents(Coin::Penny));
}
