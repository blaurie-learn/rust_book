//page 179




fn main() {
    //Yu can access elements in a string in other ways.
    //The .chars method will let you access individual unicode scalar values
    for c in "Hello world".chars() {
        println!("{}", c);
    }


    //the .bytes method will return each raw byte:
    for b in "Hello World".bytes() {
        println!("{}", b);
    }

    //keep in mind that unicode scalars may be made up of more than one byte. Getting grapheme
    //clusters from strings in complex, so this functionality is not provided by the standard
    //library.
}

