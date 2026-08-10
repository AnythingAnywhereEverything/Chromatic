// multipart-derive/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    Attribute,
    Data,
    DeriveInput,
    Fields,
    Type,
};

#[proc_macro_derive(Multipart, attributes(multipart))]
pub fn derive_multipart(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,

            _ => {
                return syn::Error::new_spanned(
                    &data.fields,
                    "Multipart requires named struct fields",
                )
                .to_compile_error()
                .into();
            }
        },

        _ => {
            return syn::Error::new_spanned(
                &input,
                "Multipart can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let generated_fields = fields.iter().map(|field| {
        let ident = field
            .ident
            .as_ref()
            .expect("named fields always have identifiers");

        let field_name = ident.to_string();

        let kind = if has_multipart_attribute(&field.attrs) {
            quote! {
                crate::application::service::media::multipart_ex::MultipartFieldKind::File
            }
        } else {
            quote! {
                crate::application::service::media::multipart_ex::MultipartFieldKind::Text
            }
        };

        let cardinality = cardinality_for_type(&field.ty);

        quote! {
            crate::application::service::media::multipart_ex::MultipartField {
                name: #field_name,
                kind: #kind,
                cardinality: #cardinality,
            }
        }
    });

    let expanded = quote! {
        impl crate::application::service::media::multipart_ex::MultipartSchema
            for #struct_name
        {
            fn multipart_fields()
                -> &'static [
                    crate::application::service::media::multipart_ex::MultipartField
                ]
            {
                &[
                    #(#generated_fields),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}

fn has_multipart_attribute(attributes: &[Attribute]) -> bool {
    attributes
        .iter()
        .any(|attribute| attribute.path().is_ident("multipart"))
}

fn cardinality_for_type(ty: &Type) -> proc_macro2::TokenStream {
    if is_vec(ty) {
        quote! {
            crate::application::service::media::multipart_ex::MultipartFieldCardinality::Many
        }
    } else if is_option(ty) {
        quote! {
            crate::application::service::media::multipart_ex::MultipartFieldCardinality::Optional
        }
    } else {
        quote! {
            crate::application::service::media::multipart_ex::MultipartFieldCardinality::Single
        }
    }
}

fn is_vec(ty: &Type) -> bool {
    type_matches_path(ty, "Vec")
}

fn is_option(ty: &Type) -> bool {
    type_matches_path(ty, "Option")
}

fn type_matches_path(ty: &Type, expected: &str) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    segment.ident == expected
}