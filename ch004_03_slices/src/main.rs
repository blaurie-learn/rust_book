//slices are another data type without ownership. 
//Slices let you reference a contiguous sequence of elements in a collection


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    //we can take a reference to a portion of the string like so
    //the type of a string slice is &str
    let hello = &s[0..5];
    let world = &s[6..11];
    let _whole_slice = &[..];    //slice of the whole string

    let first = first_word(&s);

    //s.clear();    //error at compile time since we later used some of the slices!

    println!("The values are {}, {}, {}", hello, world, first);


    //string literals are slices with the type &str.

    
    //you can take slices of any type with contiguous indeces:
    let a = [1, 2, 3, 4, 5];
    let _a_slice = &a[1..3];        //slice has the type &[i32]

}

//a better signature of first_word would use a slice instead. 
//This is better because it lets you use both String and &str values
//fn first_word(s: &str) -> &str { ... }
