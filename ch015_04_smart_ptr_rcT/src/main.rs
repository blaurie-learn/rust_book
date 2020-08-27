//In most cases, you know exactly which varaibles needs to own a given value, but there are cases
//where a single value can have multiple owners.

//Rust has Rc<T> to enable multiple ownership. (Reference counted)

enum List {
    Cons(i32, Box<List>),
    Nil
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil
}

use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use std::rc::Rc;

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    //let c = Cons(4, Box::new(a));   //compiler error !

    let ra = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));

    //Note that we use Rc::clone() as this will increase the reference count
    let rb = RcCons(3, Rc::clone(&ra));
    let rc = RcCons(4, Rc::clone(&ra));

    
    //Showing how cloning increases the reference count:
    println!("The current count is: {}", Rc::strong_count(&ra));
    {
        let rd = RcCons(5, Rc::clone(&ra));

        println!("The current count is: {}", Rc::strong_count(&ra));

    }
    println!("The current count after scope close is: {}", Rc::strong_count(&ra));

    //note that Rc<T> lets you share immutable data to different parts of your program. For
    //interior mutability, have a look at RefCell<T>, up next
}
