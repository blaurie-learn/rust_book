//The term macro refers to a family of features in rust:
//  Declarative macros with macro_rules!
//  Three kinds of procedural macros
//      -Custom #[derive] macros that specify code added with the derive attribute on structs and
//      enums
//      -Attribute-like macros that define custom attributes usable on any item
//      -Function-like macros that look like function calls but operate on the tokens specified as
//      their argument

//macros are a way or writing code that writes code (metaprogramming).
//macros are expanded before the meaning of code is interpreted, so a macro can implement a trait
//on a given type


//-------------------------------
//Declarative macros
//to clare, use the "macro_rules!" construct. Let's look at how the vec! macro works!
//#[macro_export]
//macro_rules! vec {
//    ( $( $x: expr ),* ) => {
//        {
//            let mut temp_vec = Vec::new();
//            $(
//                temp_vec.push($x);
//            )*
//            temp_vec
//        }
//    };
//}

//The #[macro_export] annotation indicates that this macro should be made available whenever the
//crate is brought in to scope
//Then there's the "macro_rules! vec {" which declares the macro and opens the macro body
//Now for the pattern:
//  -The first set of parenthesis encompases the whole pattern
//  -Then the $( ) which captures the values that match the given pattern.
//  -x$x:expr is the given pattern. This matches any rust expression
//  -the comma following the $( ) indicates a literal comma separator could optionall appear after
//  the code in the $( )
//  -The * after the comma indicates that whatever precedes the * could match 0 or more occurrences
//      So, when vec![1, 2, 3] is expanded:
//          $x pattern matches the 1, 2, and 3.
//          The macro expands to:
//          {
//              let mut temp_vec = Vec::new();
//              temp_vec.push(1);
//              temp_vec.push(2);
//              temp_vec.push(3);
//              temp_vec
//          }
//
//For more information on rust macros. See my good friend google.




//-----------------------
//procedural macros for generating code form Attributes
//Act more like functions (and are a type of procedure). They accept some code as input, operate on
//that code, and produce some code as output rather than patching against patterns and replacing
//the code with other code as declarative macros do.
//
//The definitions of procedural macros must reside in their own crate with a special crate type. 
//
//use proc_macro;
//
//#[some_attribute]
//pub fn some_name(input: TokenStream) -> TokenStream {
//}
//
//
//This defines a procedural macrothat takes a TokenStream as input and produces a TokenStream as an
//output. We'll make new crates to show the other three macro types...






//-------------
//Attribute like macros
//Are similar to custom derive macros, but instead of generating code for the derive attribute,
//they allow you to create new attributes.
//
//an example is:
//  #[route(GET, "/")]
//  fn index() { }
//
//which would be defined 
//  #[proc_macro_attribute]
//  pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
//
//Otherwise they work the same as derive macros. You create the crate with the proc-macro crate
//type and implement a function that generates the code you want.






//-----------------
//Function-like macros
//defines macros that look like function calls. More flexible than functions -- can take an
//unknowned number of arguments, for example.
//Function-like parameters also take a TokenStream and manipulate it.
//
//an example is sql!
//  let sql = sql!(SELECT * FROM posts WHERE id=1)
//
//This macro would parse the SQL statement inside and check that it is syntactically correct, which
//is more complex than macro_rules! can do. 
//It would be defined like so:
//  #[proc_macro]
//  pub fn sql(input: TokenStream) -> TokenStream { ... }
//
//


fn main() {
    println!("Hello, world!");
}
