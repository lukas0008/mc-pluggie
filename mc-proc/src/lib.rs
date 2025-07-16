use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    Ident, Lit, Token,
};

struct Property {
    key: Ident,
    _eq_token: Token![=],
    value: Lit,
}

impl Parse for Property {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Property {
            key: input.parse()?,
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn packet(
    input: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(item.clone()).unwrap();
    let name = &ast.ident;
    let (impl_generics, ty_generics, _) = ast.generics.split_for_impl();

    let input = Punctuated::<Property, Token![,]>::parse_separated_nonempty
        .parse(input)
        .unwrap();
    let mut id = None;
    let mut serialize_only = false;
    for property in input {
        match property.key.to_string().as_str() {
            "id" => {
                id = Some(property.value);
            }
            "serialize_only" => match property.value {
                syn::Lit::Bool(syn::LitBool { value, span: _ }) => {
                    serialize_only = value;
                }
                _ => panic!("Expected boolean value for 'serialize_only' property"),
            },
            _ => panic!("Unknown property: {}", property.key),
        }
    }

    let id = id.expect("Missing 'id' property");

    let item: TokenStream = item.into();

    #[cfg(feature = "serde-derive")]
    let serialize = quote! { , serde::Serialize };
    #[cfg(not(feature = "serde-derive"))]
    let serialize = quote! {};
    #[cfg(not(feature = "serde-derive"))]
    let deserialize = quote! {};
    #[cfg(feature = "serde-derive")]
    let deserialize = {
        if !serialize_only {
            quote! { , serde::Deserialize }
        } else {
            quote! {}
        }
    };

    let derive = quote! {
        #[derive(Debug #serialize #deserialize)]
    };

    #[cfg(feature = "serde-derive")]
    let code = quote! {
        #derive
        #item
        impl #impl_generics crate::packet::Packet for #name #ty_generics {
            const PACKET_ID: i32 = #id;
        }
        impl #impl_generics crate::packet::PacketSerialize for #name #ty_generics {
            fn serialize_packet(&self) -> Vec<u8> {
                use serde::Serialize;
                let mut serializer = crate::serde::serializer::Serializer::new();
                self.serialize(&mut serializer).expect("Error serializing packet");
                serializer.output
            }
            fn packet_id(&self) -> i32 {
                #id
            }
        }
    };

    code.into()
}
