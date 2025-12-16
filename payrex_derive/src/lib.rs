mod fields;

use darling::{FromMeta, ast::NestedMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct, Meta, parse_macro_input, punctuated::Punctuated, token::Comma};

use crate::fields::{ParsedPayrexAttrs, PayrexAttrs};

#[proc_macro_attribute]
pub fn payrex(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_struct = parse_macro_input!(item as ItemStruct);
    let metas = parse_macro_input!(attr with Punctuated::<Meta, Comma>::parse_terminated);
    let nested: Vec<NestedMeta> = metas.into_iter().map(NestedMeta::Meta).collect();

    let opts = match PayrexAttrs::from_list(&nested) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let fields = match &mut input_struct.fields {
        Fields::Named(f) => &mut f.named,
        _ => {
            return syn::Error::new_spanned(
                &input_struct,
                "#[payrex] only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    let parsed_opts: ParsedPayrexAttrs = opts.into();
    let mut opts = parsed_opts.set_fields(fields);

    opts.add_amount();
    opts.add_metadata();
    opts.add_description();
    opts.add_livemode();
    opts.add_timestamp();
    opts.add_currency();

    *fields = opts.fields;

    TokenStream::from(quote! {
        #input_struct
    })
}
