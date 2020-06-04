// Rust provides a number of features to manage code:
//      Packages        Cargo feature that lets you build, test and share crates
//      Crates          A tree of modules that produces a library or executable
//      Modules and use Let you control the organization, scope and privacy of paths
//      Paths           A way of naming an item, such as a struct, function, or module

// a crate is a binary or library.
// crate root is the source file that the Rust compielr starts from and makes up root module of
// crate

// cargo follows the convention
//  if the crate root is src/main.rs, this is a binary create
//  if the crate root is src/lib.rs, this is a library crate



fn main() {
    println!("Hello, world!");
}
