use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        //note mut to make the variable mutable
        //String is a growable UTF-8 encoded text.
        let mut guess = String::new();

        io::stdin()

            //again, note the &mut to pass a mutable reference
            //read_line returns a Result which is an enum of Ok and Err.
            .read_line(&mut guess)

            //this is pretty typical error handling in rust.
            .expect("Failed to read line");

        //Note how rust supports shadowing variables locally.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
