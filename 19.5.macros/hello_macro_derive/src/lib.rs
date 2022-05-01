extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

// can only be used in `struct` and `enum`
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

/// Now we can use #[route(GET, "/")] like attribute macro
/// `attr` has the attributes; e.g. GET, "/"
/// `item` has the body that the attribute is attached; e.g. fn index() {}
/// ```
/// #[route(GET, "/")]
/// fn index() {}
/// ```
///
/// The other things work same way as custom derive macros.
/// Return stream will be the generated code.
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// This is the function-like macro
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // we can see all tokens passed
    for token in input.into_iter() {
        println!("{:?}", token);
    }
    quote!(3).into()
}
