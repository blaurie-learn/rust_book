//You can also pass regular functions to to functions!

fn add_one(x: i32) -> i32 {
    x + 1
}

//Note the fn type, not to be confused with the Fn closure trait. All functions coerce to fn.
//Also note than fn implements Fn, FnMut, and FnOnce
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

//An example where you would for sure only use a function pointer is with code interfacing with
//external code (such as C).

//----------------

//There's also a "hack" that we can do that exploits an implementation detail of tuple struct enum
//variants.
//These types use () as an initializer syntax, which looks like a function call. The initializers
//are actually implemented as functions returning an instance that's constructed from their
//arguments. So we can sure that as a function pointer:
enum Status {
    Value(u32),
    Stop,
}


//---------------------
//Returning closures. Closures are represented by traits, so can't be returned directly. 
//We can get around this using the trait object syntax:
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}.", answer);

    //-----------------
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

}
