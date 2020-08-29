//Interior mutability is a pattern that lets you mutate data even when there are immutable
//references to that data.
//This pattern uses unsafe code to bend rusts rules around mutability and borrowing

//The unsafe code involved is then wrapped in a safe API and the outer type is still immutable

//RefCell<T> represents single ownership over the data it holds. Recall borrowing rules
//  -At a given time, you can have either, but not both of, one mutable reference or any number of
//  immutable references
//  -References must always be valid
//
//With RefCell<T>, these rules are enforced at runtime. If you break the rules at runtime, your
//program will panic.
//This is useful when you're sure your code follows the borrowing rules, but the compiler is being
//too conservative and rejects it.
//RefCell<T> is only useful for single threaded scenarios.
//
//Because borrowing rules are enforced at runtime, you can mutate the value inside even when the
//RefCell<T> is immutable.
//





//see the lib.rs at this point




//With regular rust, we use the & and &mut to borrow and mutably borrow. With RefCell, we use
//borrow() and borrow_mut() methods, which are part of the safe API that belongs to RefCell<T>.
//The bhorrow() method returns the smart pointer Ref<T> and borrow_mut() returns RefMut<T>.
//Both types implement Deref, so we can treat them like regular references.

//RefCell keeps track of how many times we've called borrow and borrow_mut.

//If we violate the standard borrowing rules (many immutable or one mutable), then we will panic at
//runtime!


//A common way to use RefCell<T> is with Rc<T> (which lets you have multiple owners!). If you have
//a Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners *and* that you
//can mutate.
//recall the Cons List:

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *(value.borrow_mut()) += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);


}

//The standard library has other types that offer interior mutability such as Cell<T> (Similar to
//RefCell except the value inside is copied in and out) and Mutex which offers thread safe interior
//mutability.
