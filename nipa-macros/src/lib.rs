use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn letter(input: TokenStream) -> TokenStream {
    let input_parsed = parse_macro_input!(input as syn::Ident);
    let ident = input_parsed;
    TokenStream::from(quote! {
        struct #ident;
        impl #ident {
            pub fn print_upper() {
                print!("{}", stringify!(#ident).to_uppercase());
            }

            pub fn print_lower() {
                print!("{}", stringify!(#ident).to_lowercase());
            }

            pub fn print_upper_space() {
                print!("{} ", stringify!(#ident).to_uppercase());
            }

            pub fn print_lower_space() {
                print!("{} ", stringify!(#ident).to_lowercase());
            }

            pub fn println_upper() {
                println!("{}", stringify!(#ident).to_uppercase());
            }

            pub fn println_lower() {
                println!("{}", stringify!(#ident).to_lowercase());
            }
        }
    })
}
