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
            fn new() -> Self {
                 Self
             }
        }

        impl nipa_traits::Letter for #ident {
            fn print_upper(&self) {
                print!("{}", stringify!(#ident).to_uppercase());
            }

            fn print_lower(&self) {
                print!("{}", stringify!(#ident).to_lowercase());
            }

            fn print_upper_space(&self) {
                print!("{} ", stringify!(#ident).to_uppercase());
            }

            fn print_lower_space(&self) {
                print!("{} ", stringify!(#ident).to_lowercase());
            }

            fn println_upper(&self) {
                println!("{}", stringify!(#ident).to_uppercase());
            }

            fn println_lower(&self) {
                println!("{}", stringify!(#ident).to_lowercase());
            }

            fn print_upper_dot(&self) {
                print!("{}.", stringify!(#ident).to_uppercase());
            }

            fn print_lower_dot(&self) {
                print!("{}.", stringify!(#ident).to_lowercase());
            }

            fn print_upper_space_dot(&self) {
                print!("{}. ", stringify!(#ident).to_uppercase());
            }

            fn print_lower_space_dot(&self) {
                print!("{}. ", stringify!(#ident).to_lowercase());
            }

            fn print_upper_space_dot_after(&self) {
                print!("{} .", stringify!(#ident).to_uppercase());
            }

            fn print_lower_space_dot_after(&self) {
                print!("{} .", stringify!(#ident).to_lowercase());
            }

            fn println_upper_dot(&self) {
                println!("{}.", stringify!(#ident).to_uppercase());
            }

            fn println_lower_dot(&self) {
                println!("{}.", stringify!(#ident).to_lowercase());
            }

            fn print_upper_comma(&self) {
                print!("{},", stringify!(#ident).to_uppercase());
            }

            fn print_lower_comma(&self) {
                print!("{},", stringify!(#ident).to_lowercase());
            }

            fn print_upper_space_comma(&self) {
                print!("{}, ", stringify!(#ident).to_uppercase());
            }

            fn print_lower_space_comma(&self) {
                print!("{}, ", stringify!(#ident).to_lowercase());
            }

            fn print_upper_space_comma_after(&self) {
                print!("{} ,", stringify!(#ident).to_uppercase());
            }

            fn print_lower_space_comma_after(&self) {
                print!("{} ,", stringify!(#ident).to_lowercase());
            }

            fn println_upper_comma(&self) {
                println!("{},", stringify!(#ident).to_uppercase());
            }

            fn println_lower_comma(&self) {
                println!("{},", stringify!(#ident).to_lowercase());
            }
        }
    })
}
