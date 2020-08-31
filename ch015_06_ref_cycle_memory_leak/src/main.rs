//Rust allows memory leaks with Rc<T> and RefCell<T>: it is possible to create references where
//items refer to eachother in a cycle which creates memory leaks (the reference count can' reach 0
//and so the values will not drop.

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {

    //We make a list a, then b which points to a, then modify a to point to b.
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // the derived debug printout will attempt to follow the values through and print them.
    // so it follows a to b to a to b to a to b, etc.
    //println!("a next item = {:?}", a.tail());

    //This is a demonstration of how rust can't catch all reference cycles and how the programmer
    //must intervene and be careful not to do things that can cause them.




    //You can prevent reference cycles with Weak<T> by calling Rc::downgrade and passing a
    //reference to the Rc<T>. Then, instead of increasing the Rc::strong_count, the Rc::weak_count
    //will be increased.
    //weak_count will keep track of the weak references to an object, but, unlike the strong_count,
    //doesn't need to be zero for a Rc<T> instance to be cleaned up.

    //Because a Weak<T> might have been dropped, a programmer must ensure that something is still
    //being pointed to when using. Calling Weak::upgrade will return an Option<Rc<T>>


    //in this example, we'd really like leaf to know about their parent branch, but to do so would
    //create a reference cycle if we use strong references. The parent should be a weak reference.
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent:RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
}

use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    //adding a parent as Weak<T>
    parent: RefCell<Weak<Node>>
}
