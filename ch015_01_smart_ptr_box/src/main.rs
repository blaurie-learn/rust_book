//String and Vec<T> are smart pointers.
//Smart pointers usually implemented with structs except they implement the Deref and Drop traits.
//  -Deref allows instance of smart pointer to behave like a reference so you can write code that
//  works with either a reference or a smart pointer
//  -Drop lets yu customize the code that's run when a smart pointer goes out of scope

//Most straight forward smart pointer is Box<T>. ets you store data on heap rather than stack
//used when
//  -You have a type whos size can't be known at compile time and you want to use a value of that
//  type in a context that requires an exact size
//  -when you have a large amount of data and you want to transfer ownership but sure data won't be
//  copied when hou do so.
//  -When you want to own a value and you care only that it's a type that implements a particular
//  trait rather then being a specific type


//a cons list is a recursive type (a type which contains a member variable whos type is the members
//parent type). Since this member could cause infinite recursion, the types compile time size is
//unknown.
enum List {
    //Cons(i32, List),   //Recursive type. List contains a List member. Use a pointer instead
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
