use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Field, parse_quote, punctuated::Punctuated, token::Comma};

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub(crate) struct PayrexAttrs {
    pub timestamp: bool,
    pub metadata: bool,
    pub amount: bool,
    pub livemode: bool,
    pub description: Option<String>,
    pub currency: bool,
    pub optional: bool,
}

pub(crate) struct ParsedPayrexAttrs {
    attrs: PayrexAttrs,
    pub fields: Punctuated<Field, Comma>,
    pub optional_struct: TokenStream,
}

impl From<PayrexAttrs> for ParsedPayrexAttrs {
    fn from(value: PayrexAttrs) -> Self {
        Self {
            attrs: value,
            fields: Punctuated::new(),
            optional_struct: TokenStream::new(),
        }
    }
}

impl ParsedPayrexAttrs {
    pub fn set_fields(mut self, fields: &mut Punctuated<Field, Comma>) -> Self {
        self.fields = fields.clone();
        self
    }

    pub fn add_timestamp(&mut self) {
        if self.attrs.timestamp {
            self.fields.push(parse_quote! {
                /// The time the resource was created and measured in seconds since the Unix epoch.
                pub created_at: Timestamp
            });
            self.fields.push(parse_quote! {
                /// The time the resource was updated and measured in seconds since the Unix epoch.
                pub updated_at: Timestamp
            });
        }
    }

    pub fn add_metadata(&mut self) {
        if self.attrs.metadata {
            self.fields.push(parse_quote! {
                /// A set of key-value pairs attached to the Payment. This is useful for storing additional
                /// information about the Payment.
                #[serde(skip_serializing_if = "Option::is_none")]
                pub metadata: Option<Metadata>
            });
        }
    }

    pub fn add_amount(&mut self) {
        if self.attrs.amount {
            self.fields.push(parse_quote! {
                /// The amount of the payment to be transferred to your PayRex merchant account. This is a
                /// positive integer that your customer paid in the smallest currency unit, cents. If the
                /// customer paid ₱ 120.50, the amount of the Payment should be 12050.
                ///
                /// The minimum amount is ₱ 20 (2000 in cents) and the maximum amount is ₱ 59,999,999.99
                /// (5999999999 in cents).
                pub amount: u64
            });
        }
    }

    pub fn add_livemode(&mut self) {
        if self.attrs.livemode {
            self.fields.push(parse_quote! {
                /// The value is `true` if the resource's mode is live or the value is `false` if the resource is
                /// in test mode.
                pub livemode: bool
            });
        }
    }

    pub fn add_description(&mut self) {
        if let Some(desc) = &self.attrs.description {
            let docs = match desc.as_str() {
                "refund" => "An arbitrary string attached to the Refund.",
                "payment" => {
                    "An arbitrary string attached to the Payment. Useful reference when viewing Payment from [PayRex Dashboard](https://dashboard.payrexhq.com)."
                }
                "payment_intent" => {
                    "An arbitrary string attached to the Payment Intent. Useful reference when viewing paid payments from the [PayRex Dashboard](https://dashboard.payrexhq.com)."
                }
                "webhook" => {
                    "An arbitrary string attached to the Webhook. You can use this to give more information about the Webhook resource."
                }
                "checkout_session" => {
                    "An arbitrary string attached to the CheckoutSession. Useful reference when viewing paid Payment from PayRex Dashboard."
                }
                "billing_statements" => {
                    r#"
An arbitrary string attached to the billing statement and copied over to its payment intent. This is a useful reference when viewing the payment resources associated with the billing statement from the PayRex Dashboard.

If the description is not modified, the default value is "Payment for Billing Statement <billing statement number>"
                    "#
                }
                "billing_statement_line_items" => {
                    "The description attribute describes the line item of the billing statement. It could be a product you sell or a service you provide to your customers."
                }
                _ => "",
            };

            self.fields.push(parse_quote! {
                #[doc = #docs]
                #[serde(skip_serializing_if = "Option::is_none")]
                pub description: Option<String>
            });
        }
    }

    pub fn add_currency(&mut self) {
        if self.attrs.currency {
            self.fields.push(parse_quote! {
                /// A three-letter ISO currency code in uppercase. As of the moment, we only support PHP.
                pub currency: Currency
            });
        }
    }

    fn is_option(&self, ty: &syn::Type) -> bool {
        if let syn::Type::Path(p) = ty
            && let Some(seg) = p.path.segments.last()
        {
            return seg.ident == "Option";
        }
        false
    }

    fn gen_optional_fields(&self) -> TokenStream {
        let new_fields = self.fields.iter().map(|f| {
            let field_name = &f.ident;

            let field_name = match field_name {
                Some(name) => name,
                None => return quote! {},
            };

            let doc_attrs = f.attrs.iter().filter(|attr| attr.path().is_ident("doc"));
            let original_ty = &f.ty;

            let final_ty = if self.is_option(original_ty) {
                quote! { #original_ty }
            } else {
                quote! { Option<#original_ty> }
            };

            quote! {
                #(#doc_attrs)* #[serde(skip_serializing_if = "Option::is_none")]
                pub #field_name: #final_ty
            }
        });

        quote! {
            #(#new_fields),*
        }
    }

    pub fn add_optional_struct(&mut self, ident: &Ident) {
        if self.attrs.optional {
            let optional_ident = format_ident!("Optional{ident}");
            let optional_fields = self.gen_optional_fields();
            let docs = format!(
                "Optional variant for {ident}. This is only used for responses in billing statements API."
            );
            self.optional_struct.extend(quote! {
                #[doc = #docs]
                #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
                pub struct #optional_ident {
                    #optional_fields
                }
            });
        }
    }
}
