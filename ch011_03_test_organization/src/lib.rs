//rust thinks of tests in terms of two main categories: unit and integration
//  unit - small, focused, testing one module in isolation, can test private interfaces
//  integration - external to library, use code in the same way the user would, public interface

//Unit tests should be put in the src directory inside each file that they testing. The convention
//is to create a module "tests" inside each file to contain the test functions and to annotate that
//module with #[cfg(test)]

//integration tests are external to the library. We create a "tests" directory **next to src** and
//cargo knows to look here. Check out this dir for more notes...

//modules annotated with #[cfg(test)] only compile when "cargo test" is run. They are ignored for
//"cargo build"

//this convention allows you to test internal private functions (which itself is hotly debated in
//other languages as to whether you should or not...)
//for example:
pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_test() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
