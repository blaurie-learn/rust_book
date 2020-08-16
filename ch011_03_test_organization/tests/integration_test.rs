use ch011_03_test_organization;

//here's an example where we have a common module created
mod common;

//keep in mind that crates that provide only a src/main.rs cannot have integration tests.
//For this reason, it is idiomatic to use main.rs for simple logic to call into a lib.rs file.


#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, ch011_03_test_organization::add_two(2));
}
