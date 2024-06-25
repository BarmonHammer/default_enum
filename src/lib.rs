//this is terrible, but i'm being forced to publish
//first proc macro
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

///makes a market struct for every variant of the enum which is tied to a trait.
/// that trait can be used to make default enum structs
#[proc_macro_derive(EnumVariants)]
pub fn enum_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let Data::Enum(ref enum_data) = input.data else {
        panic!("EnumVariants macro can only be applied to enums");
    };

    let enum_name = &input.ident;

    let trait_name = format!("{}Default", enum_name.to_string());
    let trait_ident = syn::Ident::new(&trait_name, Span::call_site());

    let variants = enum_data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let struct_name = format!("{}", variant_name);
        let struct_ident = syn::Ident::new(&struct_name, variant.span());

        let struct_def = quote_spanned! {variant.span()=>
            pub(crate) struct #struct_ident;
        };

        let trait_impl = quote_spanned! {variant.span()=>
            impl #trait_ident for #struct_ident {
                fn as_variant() -> #enum_name {
                    #enum_name::#variant_name
                }
            }
        };

        quote! {
            #struct_def
            #trait_impl
        }
    });

    let expanded = quote! {
        #(#variants)*
        pub(crate) trait #trait_ident {
            fn as_variant() -> #enum_name;
        }
    };

    TokenStream::from(expanded)
}
