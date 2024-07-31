use proc_macro::TokenStream;
use darling::FromMeta;
use proc_macro2::Ident;
use quote::quote;
use syn::{DataStruct, DeriveInput, Field, Fields, parse_macro_input, Path};
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

fn generate_data_model(fields: &Fields, struct_ident: &Ident) -> proc_macro2::TokenStream {
    let mut new_fields = quote!();
    let required: Path = Path::from_string("required").unwrap();
    let optional: Path = Path::from_string("optional").unwrap();
    let omitted: Path = Path::from_string("omitted").unwrap();

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
            .filter(|attr| {
                let path = attr.path();
                path != &required &&
                    path != &optional &&
                    path != &omitted
            }).collect::<Vec<_>>();
        for attr in attrs {
            if attr.path() == &required {
                new_fields.extend(quote! {
                    #(#extra_attrs)*
                    #vis #ident #colon_token #ty,
                });
                break;
            } else if attr.path() == &omitted {
                break;
            } else if attr.path() == &optional {
                new_fields.extend(quote! {
                    #(#extra_attrs)*
                    #vis #ident #colon_token Option<#ty>,
                });
            }
        }
    }

    let struct_ident = match Ident::from_string(&(struct_ident.to_string() + "DataModel")) {
        Ok(ident) => ident,
        Err(e) => panic!("{e:?}")
    };
    quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize, ::std::clone::Clone)]
        pub struct #struct_ident {
            #new_fields
        }
    }
}