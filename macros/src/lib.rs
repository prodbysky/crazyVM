use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Type};

#[proc_macro_derive(OpcodeTraits)]
pub fn opcode_traits_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = if let Data::Enum(DataEnum { variants, .. }) = input.data {
        variants
    } else {
        panic!()
    };

    let mut variant_fields = vec![];

    for variant in &variants {
        variant_fields.push(variant.fields.clone());
    }

    let from_u32_arms = variants.iter().enumerate().map(|(i, v)| {
        let ident = &v.ident;
        let fields: Vec<_> = match &variant_fields[i] {
            syn::Fields::Unnamed(n) => n.unnamed.clone().into_iter().collect(),
            syn::Fields::Unit => vec![],
            _ => unreachable!(),
        };

        let i = i as u32;

        let mut f = vec![];

        for field in fields {
            if let Type::Path(p) = field.ty {
                f.push(p.path.get_ident().unwrap().to_string());
            }
        }

        let f1: Vec<_> = f.iter().map(|ff| &**ff).collect();

        match &f1[..] {
            ["Register", "Register", "Register"] => {
                quote! {
                    #i => Opcode::#ident(value.r1(), value.r2(), value.r3()),
                }
            }
            ["Register", "Register"] => {
                quote! {
                    #i => Opcode::#ident(value.r1(), value.r2()),
                }
            }
            ["Register", "Bit13Literal"] => {
                quote! {
                    #i => Opcode::#ident(value.r1(), value.lit13()),
                }
            }
            ["Register"] => {
                quote! {
                    #i => Opcode::#ident(value.r1()),
                }
            }
            ["Bit13Literal"] => {
                quote! {
                    #i => Opcode::#ident(value.lit13()),
                }
            }
            _ => {
                quote! {
                    #i => Opcode::#ident,
                }
            }
        }
    });

    let from_opcode_arms = variants.iter().enumerate().map(|(i, v)| {
        let ident = &v.ident;
        let fields = match &variant_fields[i] {
            syn::Fields::Unnamed(n) => n.unnamed.clone().into_iter().collect(),
            syn::Fields::Unit => vec![],
            _ => unreachable!(),
        };

        let i = i as u32;

        let mut f = vec![];

        for field in fields {
            if let Type::Path(p) = field.ty {
                f.push(p.path.get_ident().unwrap().to_string());
            }
        }

        let f1: Vec<_> = f.iter().map(|ff| &**ff).collect();

        if f1[..] == ["Register", "Register", "Register"] {
            quote! {
                Opcode::#ident(r1, r2, r3)=> #i.reg_3_instruction(r1, r2, r3),
            }
        } else if f1[..] == ["Register", "Register"] {
            quote! {
                Opcode::#ident(r1, r2)=> #i.reg_2_instruction(r1, r2),
            }
        } else if f1[..] == ["Register", "Bit13Literal"] {
            quote! {
                Opcode::#ident(r1, imm)=> #i.imm_instruction(r1, imm),
            }
        } else if f1[..] == ["Register"] {
            quote! {
                Opcode::#ident(r1)=> #i.reg_1_instruction(r1),
            }
        } else if f1[..] == ["Bit13Literal"] {
            quote! {
                Opcode::#ident(imm)=> #i.jump_instruction(imm),
            }
        } else {
            quote! {
                Opcode::#ident => #i,
            }
        }
    });

    let gen = quote! {
        impl From<u32> for Opcode {
            fn from(value: u32) -> Opcode {
                match value.op() as u32 {
                    #(#from_u32_arms)*
                    _ => unreachable!(),
                }
            }
        }

        impl From<Opcode> for u32 {
            fn from(value: Opcode) -> u32 {
                match value {
                    #(#from_opcode_arms)*
                    _ => unreachable!(),
                }
            }
        }
    };

    gen.into()
}

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
