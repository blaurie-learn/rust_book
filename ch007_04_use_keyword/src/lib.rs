mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { }
    }
}


//use keyword will bring functions in to scope:
use crate::front_of_house::hosting;

//you can also use the relative path:
//use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//in rust it is not idiomatic to bring functions in to scope, but rather its parent module.
//However, for structs and enums, it is idiomatic to bring the struct or enum in to scope.


//rust doesn't allow two names in the same scope. For example, we cannot bring both
//std::fmt::Result and std::io::Result in to the same scope like so:
//use std::fmt::Result;
//use std::io.Result;
//
//Instead, you should bring in their parent module and refer to them like so:
use std::fmt;
use std::io;
fn function1() -> fmt::Result {
    // ...
}

fn function2() -> io::Result<()> {
    // ...
}

//another option is to use  the "as" keyword to alias:
use std::fmt::Result;
use std::io::Result as IoResult;

//when we bring in to scope with the use keyword, the name available in the new scope is private by
//default. To enable code that calls this code to refer to that name as if it had been defined in
//that codes scope, we can combine pub and use. This technique is called re-exporting:
pub use crate::front_of_house::hosting;

//now external code can call the add_to_waitlist() function like so:
//  hosting::add_to_waitlist();

//re-exporting is useful so you can structure your code differently internally than how you intend
//for the code to be called by other programmers.


//we can "clean up" use lists by using a nested path syntax:
use std::cmp::Ordering;
use std::io;

//is equivalent to:
use std::{cmp::Ordering, io};

//the following brings in std::io and std::io::Write:
use std::io::{self, Write};

//or we can use the glob operator to bring everything in a module in to scope:
use std::collections::*;
