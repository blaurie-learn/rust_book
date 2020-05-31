//snake case is the preferred format


fn another_function() {
    println!("in the other function");
}

//function with parameters
fn with_parms(x: i32) {
    println!("The value is: {}", x);
}

//with return
//Note that the last expression of a block will implicitely return.
fn with_ret(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Hello, world!");

    another_function();
    with_parms(5);

    println!("The result is: {}", with_ret(3, 4));
}
