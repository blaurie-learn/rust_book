//At it's simplest, a test in rust is a function that annotated with the "test" attribute #[test]

//When you run tests with "cargo test" command, Rust builds a test runner binary that runs the
//functions annotated with the test attribute and reports on whether each function passes or fails

//When generating a new library with cargo (as was done here), an initial test function is created
//right away.


//use "cargo test  to compile in test mode, resulting in a test binary. Default behavior is to run
//all tests in parallel.
//There are additional options to test. see "cargo test --help"
//
//For example: Switch to running on a single thread:
//      cargo test -- --test-threads=1
//
//By default, if a test passes, the stdout wont be displayed in the terminal, but a failure will
//show it. To enable stdout:
//      cargo test -- --show-output
//
//We can run a single test:
//      cargo test exploration
//
//Or we can filter to run multiple tests:
//      cargo test it          (runs all test containing "it")
//
//We can add an attribute #[ignore] to our tests to ignore them unless specifically requested
//Or we can run only ignored tests with:
//      cargo test -- --ignored






#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn failure() {
        panic!("Make this test fail");
    }


    //note the "use" here. Because "tests" is a regular module that follows the usual visibility
    //rules covered in chapter 7, we needed the use to bring the code below the test module in to
    //scope
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    //we also have the highly common assert_eq! and assert_ne! macros
    //although note that under the surface, they use == and !=, which means that you need to
    //implement PartialEq and Debug (to print values) traits.
    //Both PartialEq and Debug are derivable traits
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    //in addition, there are other macro signatures which allow us to specify our own message using
    //format:
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Brad");

        assert!(
            result.contains("Brad"),
            "Greeting did not contain name, value was {}.",
            result);
    }

    //for testing that a panic occurs and is expected, we have the "should_panic" attribute:
    #[test]
    #[should_panic]
    fn panics() {
        panic!("Test will be okay!");
    }

    //to look for a precise panic, we can look for the panic text with "expected"
    //this helps when looking for specific panics when a function can panic in multiple ways
    #[test]
    #[should_panic(expected = "Panic is expected!")]
    fn expected_panic() {
        panic!("Panic is expected!");
    }

    //tests can also have a signature with Result<T, E> rather than expecting panics:
    //This in turn lets you use the ? operator as a convenient way to write tests that should fail
    //Also, if your test expects to fail, you can't use should_panic with Result<T, E>. Instead you
    //should return an Err value directly when the test is meant to fail
    #[test]
    fn result_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
