//Drop trait lets you customize what happens when a value is about to go out of scope.

struct CustomSmartPointer {
    data: String,
}

//Drop has you implement a drop() function that takes a mutable pointer to self
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}.", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");


    //if you want to force a value to drop before the end of its scope, you can use std::mem::drop
    //function provided by the standard library.
    //If we attempt to call the drop traits drop method manually, the compiler will give us an
    //error because we can't disable rust automatically calling drop when scope ends.
    drop(c);
    println!("A custom pointer dropped before the end of main!");

    //note how the variables are dropped in the reverse of creation.
}
