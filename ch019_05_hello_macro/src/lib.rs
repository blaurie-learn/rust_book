pub trait HelloMacro {
    fn hello_macro();
}


//We've defined the trait that we wish to allow to derive.
//Now we need an implemetation. THe implementation needs to go in to its own crate. The convention
//for derivable macros is to have a crate that defines the trait called foo, then a sub-crate called
//foo_derive.
