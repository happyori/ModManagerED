extern crate proc_macro2;

use proc_macro::TokenStream;
use crate::command_gen::{command_gen_impl, define_cmd_impl};

pub(crate) mod data_model;
pub(crate) mod generator;
pub(crate) mod command_gen;

/// Creates a DataModel structure
///
/// Attribute required: makes the field required
/// Attribute optional: wraps the field with Option<...>
/// Attribute omitted: removes the field from DataModel
///
/// Example:
/// ```rust
/// use macros::DeriveDataModel;
/// #[derive(DeriveDataModel)]
/// pub struct User {
///   #[omitted] pub id: String,
///   #[required] pub name: String,
///   #[optional] pub username: String,
/// }
///
/// // EXPANDED
/// #[derive(serde::Serialize, serde::Deserialize, Debug)]
/// pub struct UserDataModel {
///   pub name: String,
///   pub username: Option<String>,
/// }
/// ```
#[proc_macro_derive(DeriveDataModel, attributes(required, optional, omitted))]
pub fn derive_data_model(item: TokenStream) -> TokenStream {
    data_model::derive_data_model_impl(item)
}

#[proc_macro_attribute]
pub fn generate_typescript(args: TokenStream, item: TokenStream) -> TokenStream {
    generator::generate_typescript_impl(args, item)
}

#[proc_macro]
pub fn generate_commands(item: TokenStream) -> TokenStream {
    command_gen_impl(item)
}

#[proc_macro_attribute]
pub fn define_cmd(args: TokenStream, item: TokenStream) -> TokenStream {
    define_cmd_impl(args, item)
}