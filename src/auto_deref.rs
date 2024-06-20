use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Generics, Ident, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: Ident,
    generics: Generics,
    data: Data<(), AutoDerefFieldInfo>,
    #[darling(default)]
    mutable: bool,

    #[darling(default)]
    field: Option<Ident>,
}

#[derive(Debug, FromField)]
struct AutoDerefFieldInfo {
    ident: Option<Ident>,
    ty: Type,
}

pub(crate) fn process_auto_deref(input: syn::DeriveInput) -> TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()
    else {
        panic!("AutoDeref only works on structs")
    };

    let (fd, ty) = if let Some(field) = field {
        match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
            Some(f) => (field, &f.ty),
            None => panic!("Field {} not found in struct", field),
        }
    } else {
        // if only 1 field ,use that field
        if fields.len() == 1 {
            let f = fields.iter().next().unwrap();
            (f.ident.as_ref().unwrap().clone(), &f.ty)
        } else {
            panic!("AutoDeref requires a field to be specified for structs with more than 1 field")
        }
    };

    let mut code = vec![quote! {
        impl #generics ::core::ops::Deref for #ident #generics {
            type Target = #ty;
            fn deref(&self) -> &Self::Target {
                &self.#fd
            }
        }
    }];
    if mutable {
        code.push(quote! {
            impl #generics ::core::ops::DerefMut for #ident #generics {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#fd
                }
            }
        });
    }

    quote! {
        #(#code)*
    }
}
