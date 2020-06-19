//starts at page164

//rust provides common collections in the standard library. Unlike arrays or tuples, these are
//stored on the heap

fn main() {

    let _v: Vec<i32> = Vec::new();

    //an alternative way is to use the initialization macro:
    let mut v = vec![1, 2, 3];
    
    //adding to a vector:
    v.push(5);

    //You can read a value with either the indexing syntax or the get method:
    let third: &i32 = &v[2];

    match v.get(2) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("No third element"),
    }

    
    //you have two ways so you can choose the behavior of what happens when you attempt to access
    //an element out of bounds.
    //the & with [] will panic if attempt to access an out of bounds element
    //
    //Whereas .get returns an Option<T>, so it'll return None when you access an element out of
    //bounds


    //recall the rule that states you can't have a mutable and immutable reference in the same
    //scope.
    let mut mutv = vec![1, 2, 3, 4, 5];
    let first = &mutv[0];

    mutv.push(6);
    //println!("The first element is: {}", first); //when we try to use first, the compiler stops.

    //Even though the code above looks like it should work, the way a vector works is that it may
    //need to allocate new memory when it grows, which would mean that the reference to first would
    //be pointing to invalid memory.


    //Iterating over the values in a vector
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    //we can also iterate over a mutable vector in order to make changes to elements:
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("Value after mut is: {}", i);
    }



    //we can exploit enums to enable vectors to hold multiple types of values:
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //there are other methods which will be covered later.
    
    //be sure to check out the documentation for other methods that vectors have

}   // <-- all vectors declared go out of scope and are freed


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}



