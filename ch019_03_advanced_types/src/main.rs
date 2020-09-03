//Newtype pattern for things like hiding implementations:
//wrap a type in a tuple struct:
struct NewType(Vec<i32>);




//Type aliases give an existing type another name:
type WholeNumbers = i32;

//The main use for type aliases is to reduce repetition:
type Thunk = Box<dyn Fn() + Send + 'static>

type Result<T> = std::result::Result<T, std::io::Error>


//The "never type" that never returns
//Rust has a special type named "!" known as an empty type. It stands in place of the return type
//when a function will never return:
fn bar() => ! { }





fn main() {
    println!("Hello, world!");
}
