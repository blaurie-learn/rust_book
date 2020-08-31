//Rust actually have very few concurrency options. Most were previously talked about!

//There are two more concurrency concepts embedded in the language: the "std::marker" traits "Sync"
//and "Send"

//The "Send" trait allowss transferrence of ownership between threads.
//Any type that is composed entirely of "Send" types is automatically marked as Send. Most types in
//rust are Send with a few exceptions (such as Rc<T>, which we noted earlier)


//The "Sync" trait indicates that it is safe for the type implementing Sync to be references from
//multiple threads.
//In other words, any type T is Sync if &T (a reference to T) is Send, meaning the reference can be
//sent safely to another thread.
//Any tye composed entirely of Sync is automatically marked Sync.


//Implementing Send and Sync manually is unsafe.


fn main() {
    println!("Hello, world!");
}
