//Note that we did updates to this projects Cargo.toml.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


//hello_macro_derive will be called when a user of our library specifies:
//#[derive(HelloMacro)]
//syn parses ruse code -- no easy task
//quote turns syn data back in to rust code
//
//The struct we get back from syn:
//DeriveInput {
//  // --snip
//
//  ident: Ident {
//      ident: "pancakes",
//      span: #0 bytes(95..103)
//  },
//  data: Struct(
//      DataStruct {
//          struct_token: Struct,
//          fields: Unit,
//          semi_token: Some(Semi)
//      }
//  )
//}
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    //Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast = syn::parse(input).unwrap();

    //build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
