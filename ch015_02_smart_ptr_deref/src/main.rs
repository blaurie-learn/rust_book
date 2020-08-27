//implementing "Deref" lets you customize the behavior of the dereference operator "*".

use std::ops::Deref;

fn main() {
    //A regular reference is a type of pointer:
    let x = 5;
    let y = &x;
    let z = Box::new(x);    //Instance of Box pointing to a heap location set to the value in x
    let m = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);  //dereference follows reference to "x"
    assert_eq!(5, *z);  //dereference follows reference to heap
    assert_eq!(5, *m);

    //deref coercion example:
    //It lets us coerce from &{MyBox<String>} to &str
    //Because MyBox implement Deref, rust can create an &String from MyBox<String> by calling
    //deref(). Then String implements Deref, so &String calling deref on it returns a &str.
    let name = MyBox::new(String::from("Brad"));
    hello(&name);

    //if rust didn't have deref coercion, we'd have to write this instead:
    hello(&(*name)[..]); //Dereference name for String, then slice from beginning to end and reference it.
}

//Tuple struct with one element of type T.
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//To enable the use of the * operator on MyBox, we need to implement Deref
impl<T> Deref for MyBox<T> {
    type Target = T;            //Associated type for Deref Trait

    //deref will return the value we want to access when the * operator is used on this object
    fn deref(&self) -> &T {
        &self.0
    }
}

//Without the "deref" trait, the compiler cna only dereference "&" references. The deref method
//gives the compiler the ability to take a value of any type that implements Deref and call the
//deref method to get a "&" reference that it knows how to dereference.

//The compielr swaps *m above for *(m.deref()) which lets us write code that functions identially
//whether our code uses a regular reference or implements deref()

//the reason deref returns a reference is that the * outside of the deref -- *(m.deref()) -- is
//still necessary due to ownership. If we didn't return a reference, a move would occur and we'd
//have to give up ownership of interior values!


//Implicit deref coercions.
//Works only on types that implement Deref. Deref coercion converts Deref types into a reference to
//another type: for example, &String will convert to &str because String implements the Deref trait
//such that it return &str
//Deref Coercion happens automatically when we pass a reference to a particular types value as an
//argument to a function or method that doesnt match the parameter type in the function or method
//definition.
fn hello(name: &str) {
    println!("Hello, {}", name);
}
//deref cosrcion needs to be resolvable at compile time, so there i sno runtime penalty to using
//it.


//deref Coercion with mutability
//DerefMut trait can override the * operator on mutable references.
//Rust does deref coercion in the following ways:
//  From &T to &U when T: Deref<Target=U>
//  From &mut T to &mut U when T: DerefMut<Target=U>
//  From *mut T to &U when T: Deref<Target=U>
//
//  You can see rust will coerce a mutable reference to an immunatble one, but it won't do the
//  other way around.
