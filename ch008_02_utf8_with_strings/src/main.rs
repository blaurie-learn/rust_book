//page 170
//
//
//

//core rust only has one string type, the string slice: &str, which is a reference to a UTF-8
//encoded string somewhere

//The standard library provides String which is a growable, mutable, owned, UTF-8 encoded string.

//The standard library also provides OsSrtring, OsStr, CString, and CStr


fn main() {
    let mut s = String::new();

    let data = "initial contents";

    //any type that implements the Display trait will have a to_string() method:
    let s = data.to_string();

    //we've already seen String::from()
    let s = String::from("initial contents");

    //String can grow:
    let mut s = String::from("foo");
    s.push_str("bar");

    //push_str takes a string slice because we don't necessarily want to take ownership of the
    //parameter. For example, the following wouldn't work due to s2 ownership being gone:
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}!", s2);

    //another method on String is .push, which takes a single character.


    //often you want to contatenate strings. use the + Operator or the format! macro:
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;      //note s1 has been moved and can no longer be used
    //the add function is defined like (except with generics):
    //      fn add(self, s: &str) -> String { ... }
    // Note how s2 is a String and add takes &str. &String can be coerced in to &str


    //when we're dealing with lots of strings, or a more complex string format, the format! macro
    //is easier to use:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{}-{}-{}", s1, s2, s3);


    //Rust string don't support direct indexing like other langauges. This has to do with how
    //strings are stored in memory
    //Many characters cannot be properly represented with just 8 bytes per character, so if you
    //were to try to index for a single "character" you wouldn't get all the information for the
    //character.
    
    //There are 3 ways to look at strings in rust:
    //  Bytes
    //  Scalar Values
    //  Grapheme Clusters (Closest thing to what we call letters)
    //
    //Because a "character" might not actually land on 8 byte boundaries, rust asks you to be more
    //specific when you are looking at specific parts of a string by using "ranges":

    let hello = "????????????";
    let s = &hello[0..4];
    //here, s will be a reference to the first 4 bytes of the string hello.
    
    //be cautious, however, as ranges can cause your program to crash when you don't land on a
    //boundary
}
