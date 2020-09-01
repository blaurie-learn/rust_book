//to switch to unsafe rust, use the "unsafe" keyword, then start a new block
//Unsafe rust gives you 5 actions that safe rust does not
//  Dereference a raw pointer
//  Call an unsafe function or method
//  Access or modify a mutable static variable
//  Implement an unsafe trait
//  Access fields of Unions
//
//Unsafe doesn't turn off thr borrow checker and doesn't necessarily mean that the code in the
//block is dangerous, just that the compiler might reject safe code that the programmer knows is
//safe.
//
//It's best to isolate unsafe code and wrap it in a safe API in general.


fn main() {
    //Dereferencing a raw pointer:
    {
        //Unsafe rust has two types called raw pointers, which are similar to references.
        //Raw pointers can be mutable or immutable and are written "*mut T" and "*const T"
        //respectively.
        //  Note that the asterisk isnt a dereference here, it is part of the type
        //
        //Different from references and smart pointers, raw pointers:
        //  -Are allowed to ignore borrowing rules by having both immutable and mutable pointers or
        //  multiple mutable pointers to the same location
        //  -Arent guaranteed to point to valid memory
        //  -Are allowed to be null
        //  -Don't implement any automatic cleanup

        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;      //We can create pointers in safe rust
                                            //Just cant dereference them.

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }


    //calling an unsafe function
    {
        //an unsafe function is one with an unsafe contract to follow. One should read and follow
        //the documentation of unsafe functions in order to avoid undefined behavior.
        unsafe fn dangerous() { }

        unsafe {
            dangerous();
        }
    }

    //Just because a function contains unsafe code doesn't meant it needs to be marked unsafe.
    //Wrapping unsafe code in a safe function is a common abstraction.

    //Looking at the split_at_mut function
    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);     //split vector in to two vectors at a given index

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        //This function cannot be implemented in safe rust (attempting to take tow borrows from the
        //same vector). Rusts borrow checker doesn't understand you are taking borrows from
        //different parts of the array.
    }


    //when using extern functions to call external code, for example, a C library
    

    //When accessing or modifying a static variable
    {
        //In rust, global variables are called static variables.
        println!("{}", HELLO_WORLD);

        add_to_count(3);

        unsafe {
            println!("{}", COUNTER);
        }
    }




    //implementing an unsafe trait
    {
        //a trait is unsafe when at least one of its methods has some invariant that the compiler
        //can't verify.
        //We can declare a trait is unsafe by putting the keyword unsafe before its definition,
        //then, we must put unsafe before the implementation block as well.
    }


    //Accessing fields of a union. Unions are similar to structs except that only one declared
    //fields is used an a particular instance. Rust can't guarantee the type of data being stored,
    //so this is unsafe.
}

static HELLO_WORLD: &str = "Hello World";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
