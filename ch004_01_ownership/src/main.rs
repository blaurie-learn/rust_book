//in fairly typical fashion, data on the stack must have a known size at compile time. Data on the
//heap doesn't need to be a fixed size.

//in general, stack is faster than heap for a variety of reasons (having to follow pointers, OS
//allocation time, etc).

fn main()
{                           //s is not valid here
    let s = "hello";        //s is valid from this point forward

    //do stuff with s
    
    {
        let mut st = String::from("hello");     //heap allocated string
        st.push_str(", world!");                //this string is mutable 

        //each allocate must be paired with a free. In rust, the memory is returned when 
        //its variable goes out of scope
        

        //momve semantics:
        //When we move a pointer from one variable to another, the original variable can no longer
        //be used:
        let st1 = st;           //st becomes invalid.
        //println!("st is: {}", st); doesn't compile because st was moved to st1

        
        //we can always create a clone if we need two variables with the same data:
        let st2 = st1.clone();
        println!("st1 is: {}, st2 is: {}", st1, st2);

        //rust has a Copy trait that can be implemented which will make a value usable after
        //assignment. Copy cannot be implemented on a type if it, or any of its parts implement the
        //Drop trait.

    }   //st goes out of scope and is freed, calling the objects "drop" function


    function_test();
}                           //this scope is over. s is no longer valid


fn function_test() {
    let s = String::from("hello");      //s comes in to scope

    takes_ownership(s);                 //s's value moves in to the function...
                                        //and is no longer valid as of here

    let x = 5;                          //x comes in to scope

    makes_copy(x);                      //x would move into the function, but i32 is Copy, so
                                        //is is still valid here
}

fn takes_ownership(some_string: String) {               //some_string
    println!("Took ownership of: {}", some_string);
}   //some_string goes out of scope. Since it is owned in this scope, Drop is called mem freed

fn makes_copy(x: i32) {         //x comes in to scope
    println!("Copy of: {}", x);
}   // x goes out of scope. nothing special happens (aside form typical stack stuff)


//similarly, returns can give ownership:
fn gives_ownership() -> String {
    String::from("ownership given")
}

fn takes_and_gives(some_str: String) -> String {
    some_str
}
