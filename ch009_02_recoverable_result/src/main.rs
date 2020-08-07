//Most errors rent serious enough to require the program to stop entirely. Sometimes, it's for a
//reason that you can respond to, or perform a more graceful exit or alternate suggestion.

//Recall that the Result enum is defined as having two variants:
//enum Result<T, E> {
//  Ok(T),
//  Err(E),
//}

use std::fs::File;
use std::io::ErrorKind;


//fn main() {
    //let f = File::open("hello.txt");

    //let f = match f {
    //    Ok(p) => p,
    //    Err(e) => panic!("uh oh"),
    //};

    //let f = match f {
    //    Ok(file) => file,
    //    Err(error) => match error.kind() {
    //        ErrorKind::NotFound => match File::create("hello.txt") {
    //            Ok(fc) => fc,
    //            Err(e) => panic!("Problem creating the file: {:?}", e),
    //        },
    //        other_error => {
    //            panic!("Problem opening the file");
    //        }
    //    },
    //};
    

    //The above is a bit much for nested matches. In later chapters, we'll see closured which can
    //clean it up a bit:
    //let f = File::open("hello.txt").unwrap_or_else(|error| {
    //    if error.kind() == ErrorKind::NotFound {
    //        File::create("hello.txt").unwrap_or_else(|error| {
    //            panic!("Problem creating.the file. {:?}", error);
    //        })
    //    } else {
    //        panic!("Problem opening the file: {:?}", error)
    //    }
    //});
    

    //match does a lot but can be a bit verbose. Result provides a number of methods to do various
    //tasks
    //with unwrap(), if result is okay, it'll return the value in Ok, if it is Err, then unwrap
    //will panic:
    //let f = File::open("hello.txt").unwrap();

    //another method, expect(), lets us customize the error message:
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");
//}

//when you're writing a function that file fail, instead of handling the error, it is often better
//to propagate it instead
use std::io;
use std::io::Read;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//the ? operator gives us a shortcut for propagating errors:
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
//errors propagated this way do have a small difference in that they go through the "from" function
//defined in the From trait in the standard library


//the ? operator requires the function to have a return type of Result.
//main function then requires a special signature to use ? propagation:
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
