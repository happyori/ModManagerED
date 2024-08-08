use proc_macro::TokenStream;
use std::sync::Mutex;
use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use lazy_static::lazy_static;
use quote::quote;
use syn::{Attribute, ItemFn, parse_macro_input, Visibility};

type CommandMap = Vec<String>;
lazy_static! {
    static ref CMDS: Mutex<Option<CommandMap>> = Mutex::new(Some(Default::default()));
}

#[derive(Debug, FromMeta)]
struct DefineCmdArg {
    #[darling(default)]
    ignore_validation: bool,
}

pub fn define_cmd_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };
    let DefineCmdArg { ignore_validation } = match DefineCmdArg::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let parsed = parse_macro_input!(item as ItemFn);
    if !ignore_validation && parsed.attrs.iter().any(|attr| attr != AttrStr("tauri::command")) {
        panic!("Define CMD macro has to be used on tauri::command")
    }
    if !ignore_validation {
        match parsed.vis {
            Visibility::Restricted(_) | Visibility::Inherited => {
                panic!("Define CMD macro has to be used on a public function");
            }
            _ => ()
        }
    }

    let ident = parsed.sig.ident.to_string();
    if let Some(map) = CMDS.lock().unwrap().as_mut() {
        map.push(ident);
    }

    let output = quote! {
        #parsed
    };
    output.into()
}

struct AttrStr(&'static str);
impl PartialEq<AttrStr> for &Attribute {
    fn eq(&self, other: &AttrStr) -> bool {
        let split = other.0.split("::").collect::<Vec<_>>();
        let segments = &self.path().segments;
        for (segment, other_segment) in segments.iter().zip(split).rev() {
            if segment.ident != other_segment {
                return false;
            }
        }
        true
    }
}

pub fn command_gen_impl(_item: TokenStream) -> TokenStream {
    return if let Some(map) = CMDS.lock().unwrap().take() {
        let str_iter = map.clone();
        let iter = map.into_iter().map(|v| syn::Ident::from_string(&v).unwrap());
        for identifier in str_iter{
            
        }
        (quote! {
            ::tauri::generate_handler![
                #(#iter),*
            ]
        }).into()
    } else {
        (quote! {
            ::tauri::generate_handler![]
        }).into()
    };
}