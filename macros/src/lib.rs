use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

#[proc_macro_derive(RegisterTraits)]
pub fn register_traits_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the enum
    let name = input.ident;

    // Ensure the input is an enum
    let variants = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        return syn::Error::new_spanned(name, "RegisterTraits can only be used on enums")
            .to_compile_error()
            .into();
    };

    // Generate match arms for the `From<u32>` implementation
    let from_u32_arms = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        let i = i as u32;
        quote! {
            #i => Register::#variant,
        }
    });

    // Generate match arms for the `From<Register>` implementation
    let from_register_arms = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        quote! {
            Register::#variant => #i as u32,
        }
    });

    // Generate match arms for the `fmt::Display` implementation
    let fmt_arms = variants.iter().map(|v| {
        let variant = &v.ident;
        let variant_str = variant.to_string();
        quote! {
            Register::#variant => write!(f, #variant_str),
        }
    });

    let gen = quote! {
        impl From<u32> for Register {
            fn from(value: u32) -> Register {
                match value {
                    #(#from_u32_arms)*
                    _ => unreachable!(),
                }
            }
        }

        impl From<Register> for u32 {
            fn from(value: #name) -> u32 {
                match value {
                    #(#from_register_arms)*
                    _ => unreachable!(),
                }
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    #(#fmt_arms)*
                    _ => unreachable!(),
                }
            }
        }
    };

    gen.into()
}
