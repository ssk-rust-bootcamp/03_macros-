use proc_macro::TokenStream;
use quote::quote;

// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    println!("input: {:#?}", input);

    let ident = input.ident;

    //get enum varinats
    let varinats = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works with enums"),
    };

    //for each variant, get the ident and fields
    let from_impls = varinats.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                //only support one field for now
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("shouled have one field");
                    let ty = &field.ty;

                    quote! {
                        impl From<#ty> for #ident {
                            fn from(f: #ty) -> Self {
                                #ident::#var(f)
                            }
                        }
                    }
                }
            }
            syn::Fields::Named(_) => quote! {},
            syn::Fields::Unit => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
    .into()
}
