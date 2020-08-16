//every reference has a lifetime, or a scope for which a reference is valid

//we need to annotate lifetimes when multiple lifetimes are possible
//main aim is to prevent dangling references


fn main() {
    let r;                                                      // ---------+-- 'a
    {                                                           //          |
        let x = 5;                                              // -+-- 'b  |
        r = &x;                                                 //  |       |
    }                                                           // -+       |
                                                                //          |
    //using r now is problematic since x is ut of scope         //          |
    //println!("r is a dangling pointer! {}", r);               // (lifetime continues to func end)

    //in the folliwing scenario since x's lifetime will be longer than r's, we can use r wherever
    //we would use x successfully.
    let x = 5;
    let r = &x;
    println!("Not dangling! {}", r);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest is: {}", result);
    }
}

//neither we, nor rust can tell what the actual lifetimes of x, y, and the return are. So without
//specifying liftimes here, rust complains that the return is expecting a lifetime parameter.
//
//&i32          -a reference
//&'a i32       -a reference with an explicit lifetime
//&'a mut i32   -a mutable reference with an explicit lifetime
//
//by adding a lifetime parameter, we declare that the return will have a lifetime which is equal to
//the smaller of the lifetimes between x and y.
//The longest function as used above would give "result" a lifetime that matchest "string2" since
//its lifetime is the smallest between string1 and string2.
//If we were to try to wrap the "longest" call up in its own scope, we would get a compile time
//issue with lifetimes.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//the way in which lifetimes need to be specified depends on what a function is doing.
//For example, if longest only ever returns x, then we wouldn't need to specify a lifetime on y.
//This is declaring that y doesn't have a lifetime relationship with x.

//also consider how it prevents dangling references:
fn wont_compile<'a>(x: &'a str, y: &str) -> &'a str {
    let result = String::from("really long string.");     //will be cleaned up at the end of the function
    result.as_str()                         //so we can't compile since this is an invalid reference!
}

//lifetime annotation in structs
//if a struct has a reference member, then it is required to specify lifetimes.
//owned members don't require a lifetime since their lifetime is guaranteed to match the struct.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//there are 3 cases where rust will implicitely insert lifetiems for you:
//1) Each parameter of a function gets its own lifetime parameter
//2) If there is exactly 1 input lifetime, that lifetime is assigned to all output lifetimes
//3) If there are multiple input lifetime parameters, but one is &self  or &mut self, the lifetime
//   of &self is assigned to all output lifetime parameters


//when we impl methods on a struct with lifetimes, we use the same syntaxas generic type
//parameters.
//Lifetime names for struct fields always needo to be declared after the impl keyword and then ised
//after the structs name, because those lifetimes are part of thes tructs type.
//lifetimes in methods might be a part of the structs fields, or they might be independant.
impl<'a> ImportantExcerpt<'a> {

    //There are two input lifetimesso rust applies the first lifetime elision rule and give both
    //&self and announcement their own lifetimes.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//there is also a special "'static" lifetime, which means that this reference can live for the
//entire duration of the program.
//All string literals have the 'static lifetime
//  let s: &'static str = "I have a static lifetime";


//here is an example of Generic Type parameters, Trait bounds and lifetimes all together:
use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
