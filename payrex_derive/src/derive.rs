use std::{collections::HashMap, sync::LazyLock};

use darling::FromField;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident, punctuated::Punctuated, token::Comma};

use crate::utils::{get_option_inner, is_type, is_type_deep};

static MAP_DESCRIPTION: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("metadata", "Sets metadata in the query parameters."),
        (
            "description",
            "Sets the description in the query parameters.",
        ),
        ("currency", "Sets the currency in the query parameters."),
        ("amount", "Sets the amount in the query parameters."),
    ])
});

#[derive(Debug, FromField)]
#[darling(attributes(payrex))]
pub(crate) struct PayrexFieldReceiver {
    #[darling(default)]
    pub description: Option<String>,
}

fn gen_optional_function(
    field: &Field,
    receiver: PayrexFieldReceiver,
    functions: &mut TokenStream2,
) {
    let ident = &field.ident;
    let ty = &field.ty;

    let inner_ty = get_option_inner(ty).unwrap();
    let docs = receiver.description;
    if is_type_deep(inner_ty, "String") {
        functions.extend(quote! {
            #[doc = #docs]
            pub fn #ident(mut self, #ident: impl Into<#inner_ty>) -> Self {
               self.#ident = Some(#ident.into());
               self
            }
        });
    } else {
        functions.extend(quote! {
            #[doc = #docs]
            pub fn #ident(mut self, #ident: #inner_ty) -> Self {
               self.#ident = Some(#ident);
               self
            }
        })
    }
}

fn gen_required_functions(ident: &Ident, fields: Punctuated<Field, Comma>) -> TokenStream2 {
    let docs = format!("Creates a new [`{}`] instance.", ident);
    if fields.is_empty() {
        return quote! {
            #[doc = #docs]
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }
        };
    }

    let fn_args = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        if is_type(ty, "String") {
            quote! { #name: impl Into<#ty> }
        } else {
            quote! { #name: #ty }
        }
    });

    let fn_body_assignments = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        if is_type(ty, "String") {
            quote! { #name: #name.into() }
        } else {
            quote! { #name }
        }
    });

    quote! {
        #[doc = #docs]
        #[must_use]
        pub fn new(#(#fn_args),*) -> Self {
            Self {
                #(#fn_body_assignments),*,
                ..Default::default()
            }
        }
    }
}

pub fn derive_handler(input: &DeriveInput) -> TokenStream {
    if let Data::Struct(data) = &input.data
        && let Fields::Named(fields) = &data.fields
    {
        let ident = &input.ident;
        let mut optional_functions = TokenStream2::new();
        let mut required_fields: Punctuated<Field, Comma> = Punctuated::new();
        for field in &fields.named {
            let field_ty = &field.ty;
            if let Ok(mut receiver) = PayrexFieldReceiver::from_field(field) {
                if let Some(field_ident) = &field.ident {
                    let ident_str = field_ident.to_string();
                    match ident_str.as_str() {
                        "metadata" | "description" | "currency" | "amount"
                            if is_type(field_ty, "Option") =>
                        {
                            receiver.description = MAP_DESCRIPTION
                                .get(ident_str.as_str())
                                .map(|desc| (*desc).to_string());
                        }
                        _ => (),
                    }
                }

                if receiver.description.is_some() {
                    gen_optional_function(field, receiver, &mut optional_functions);
                } else if !is_type(field_ty, "ListParams") {
                    required_fields.push(field.clone());
                }
            }
        }

        let new_fn = gen_required_functions(ident, required_fields);

        return quote! {
            impl #ident {
                #new_fn

                #optional_functions
            }
        }
        .into();
    }
    TokenStream::new()
}
