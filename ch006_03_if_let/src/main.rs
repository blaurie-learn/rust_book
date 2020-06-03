//When match is too wordy, we can use if let


fn main() {
    let some_u8 = Some(0u8);
    match some_u8 {
        Some(3) => println!("three"),
        _ => (),
    }

    // This match can also be written like so, but notice you lose the exhaustive checking
    if let Some(3) = some_u8 {
        println!("three");
    }
}
