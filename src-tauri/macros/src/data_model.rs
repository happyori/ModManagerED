use proc_macro::TokenStream;

use darling::FromMeta;
use proc_macro2::Ident;
use quote::quote;
use syn::{Attribute, DataStruct, DeriveInput, Field, Fields, parse_macro_input, Path};
use syn::Data::Struct;

pub(crate) fn derive_data_model_impl(input: TokenStream) -> TokenStream {
    let original_struct = parse_macro_input!(input as DeriveInput);

    let DeriveInput { data, ident, .. } = original_struct.clone();

    if let Struct(data_struct) = data {
        let DataStruct { fields, .. } = data_struct;
        let mut output = quote!();

        let generated_data_model = generate_data_model(&fields, &ident);
        output.extend(quote! { #generated_data_model });
        output.into()
    } else { panic!("DeriveDataModel can only be used on structs!") }
}

enum FieldType { Required, Optional }
struct FieldData(Ident, FieldType);

fn generate_data_model(fields: &Fields, struct_ident: &Ident) -> proc_macro2::TokenStream {
    let mut new_fields = quote!();
    let required: Path = Path::from_string("required").unwrap();
    let optional: Path = Path::from_string("optional").unwrap();
    let omitted: Path = Path::from_string("omitted").unwrap();

    let is_data_model_attr = |attr: &Attribute| {
        let path = attr.path();
        path == &required ||
            path == &optional ||
            path == &omitted
    };

    let mut field_idents = vec![];

    for Field {
        attrs,
        vis,
        ident,
        colon_token,
        ty,
        ..
    } in fields {
        let Some(ident) = ident else { panic!("Failed to get struct field identifier") };

        let extra_attrs = attrs
            .iter()
            .filter(|attr| !is_data_model_attr(*attr))
            .collect::<Vec<_>>();

        if !attrs.iter().any(is_data_model_attr) {
            field_idents.push(FieldData(ident.clone(), FieldType::Required));
            new_fields.extend(quote! {
                #(#extra_attrs)*
                #vis #ident #colon_token #ty,
            });
            break;
        }
        for attr in attrs {
            if attr.path() == &required {
                field_idents.push(FieldData(ident.clone(), FieldType::Required));
                new_fields.extend(quote! {
                    #(#extra_attrs)*
                    #vis #ident #colon_token #ty,
                });
                break;
            } else if attr.path() == &omitted {
                break;
            } else if attr.path() == &optional {
                field_idents.push(FieldData(ident.clone(), FieldType::Optional));
                new_fields.extend(quote! {
                    #(#extra_attrs)*
                    #vis #ident #colon_token Option<#ty>,
                });
                break;
            }
        }
    }

    let new_struct_ident = match Ident::from_string(&(struct_ident.to_string() + "DataModel")) {
        Ok(ident) => ident,
        Err(e) => panic!("{:?}", e)
    };
    let from_impl = build_from_impl(&new_struct_ident, &struct_ident, field_idents);
    quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize, ::std::clone::Clone, Default)]
        pub struct #new_struct_ident {
            #new_fields
        }
        #from_impl
    }
}

fn build_from_impl(new_struct: &Ident, old_struct: &Ident, fields: Vec<FieldData>) -> proc_macro2::TokenStream {
    let mut field_convert = quote! {};

    for FieldData(ident, typ) in fields {
        match typ {
            FieldType::Required => {
                field_convert.extend(quote! {
                    #ident: value.#ident,
                });
            }
            FieldType::Optional => {
                field_convert.extend(quote! {
                    #ident: Some(value.#ident),
                })
            }
        }
    }

    quote! {
        impl From<#old_struct> for #new_struct {
            fn from(value: #old_struct) -> #new_struct {
                Self {
                    #field_convert
                }
            }
        }
    }
}