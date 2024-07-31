use proc_macro::TokenStream;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{absolute, PathBuf};

use darling::{ast, Error, FromDeriveInput, FromField, FromMeta};
use darling::ast::NestedMeta;
use quote::quote;
use regex::RegexBuilder;
use syn::{GenericArgument, ItemStruct, parse_macro_input, Path, PathArguments, Type, TypePath};

#[derive(Debug, FromMeta)]
struct GeneratorArgs {
    directory: PathBuf,
}

#[derive(FromField, Debug, Clone)]
#[darling(attributes(gen))]
struct GeneratorField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    rename: Option<String>,
    #[darling(default)]
    ignored: bool,
    #[darling(default)]
    import: bool,
    #[darling(default)]
    any: bool,
    typed_as: Option<Path>,
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(gen), supports(struct_any))]
struct GeneratorInputReceiver {
    ident: syn::Ident,
    data: ast::Data<(), GeneratorField>,
    rename: Option<String>,
}

impl ToTypescript for syn::Type {
    fn to_typescript(self) -> Result<String, Error> {
        let not_implemented_error = Error::custom("This type is not implemented yet or is incompatible with typescript");
        println!("Converting type {:#?} to typescript", &self);
        match self {
            Type::Array(v) => { Ok(format!("{}[]", v.elem.to_typescript()?)) }
            Type::Ptr(v) => { Ok(v.elem.to_typescript()?) }
            Type::Reference(v) => { Ok(v.elem.to_typescript()?) }
            Type::Paren(v) => { Ok(v.elem.to_typescript()?) }
            Type::Slice(v) => { Ok(format!("{}[]", v.elem.to_typescript()?)) }
            Type::Tuple(v) => {
                let inner_types = v.elems
                    .into_iter()
                    .map(|p| p.to_typescript())
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");
                Ok(format!("[{inner_types}]"))
            }
            Type::Path(v) => {
                println!("Detected path type... Converting");
                let last_segment = v.path.segments.last().ok_or(not_implemented_error.clone())?;
                let ident_string = last_segment.ident.to_string();
                println!("Identifier -> {ident_string}");

                // Handle Vectors
                if ident_string == "Vec" && !last_segment.arguments.is_empty() {
                    println!("Detected Vector");
                    return match &last_segment.arguments {
                        PathArguments::AngleBracketed(pargs) => {
                            let inner_type = pargs
                                .args
                                .iter()
                                .filter_map(|a| {
                                    let GenericArgument::Type(ty) = a else { return None; };
                                    Some(ty)
                                }).last()
                                .ok_or(Error::custom("Vector should have a type"))?;
                            Ok(format!("Array<{}>", inner_type.clone().to_typescript()?))
                        }
                        PathArguments::None => { Err(Error::custom("Vector has to have a type")) }
                        PathArguments::Parenthesized(_) => { Err(not_implemented_error) }
                    };
                }

                //Handle Numbers
                let number_regex = RegexBuilder::new(r#"^([uif])(\d{1,3}|size)"#)
                    .build()
                    .expect("should build numbers regex");
                if number_regex.is_match(&ident_string) {
                    return Ok("number".to_string());
                }

                //Handle Strings
                if ident_string.starts_with("String") || ident_string.starts_with("str") {
                    return Ok("string".to_string());
                }

                //Handle Boolean
                if ident_string.starts_with("bool") {
                    return Ok("boolean".to_string());
                }

                println!("Failed to recognize type proceeding with passing");
                Ok(ident_string)
            }
            Type::ImplTrait(_) => Err(not_implemented_error),
            Type::Group(_) => Err(not_implemented_error),
            Type::Macro(_) => Err(not_implemented_error),
            Type::Never(_) => Ok("never".to_string()),
            _ => Ok("any".to_string())
        }
    }
}

impl ToTypescript for GeneratorField {
    fn to_typescript(self) -> Result<String, Error> {
        let name = self.name();
        let Self {
            ty,
            ignored,
            any,
            typed_as,
            ..
        } = self;
        if ignored { return Ok("".into()); }
        if any { return Ok(format!("{name}: any")); }
        if let Some(typ) = typed_as {
            let syn_type = TypePath { path: typ, qself: None };
            return Ok(format!("{name}: {}", Type::Path(syn_type).to_typescript()?));
        }

        Ok(format!("{name}: {}", ty.to_typescript()?))
    }
}

impl ToTypescript for GeneratorInputReceiver {
    fn to_typescript(self) -> Result<String, Error> {
        let Self {
            ident,
            data,
            rename
        } = self;
        let name = rename.unwrap_or(ident.to_string());
        let fields = data.take_struct().expect("Should never be enum").fields;
        let import_lines = fields
            .iter()
            .filter(|f| f.import)
            .map(|f| f.ty.clone().to_typescript())
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|type_name| format!("import type {{ {type_name} }} from './{type_name}';"))
            .reduce(|accum, line| format!("{accum}\n{line}"))
            .unwrap_or("".to_string());

        let ts_fields = fields
            .into_iter()
            .filter(|f| !f.ignored)
            .map(|f| f.to_typescript())
            .collect::<Result<Vec<_>, _>>()?
            .join(";\n    ");

        let result = format!("{import_lines}\nexport interface {name} {{\n    {};\n}}\n", ts_fields.trim_end());

        Ok(result)
    }
}

impl GeneratorInputReceiver {
    fn name(&self) -> String {
        self.rename
            .clone()
            .or(Some(self.ident.to_string()))
            .expect("Generator should have a name or a rename")
    }
}

impl GeneratorField {
    fn name(&self) -> String {
        self.rename
            .clone()
            .or(self.ident.clone().map(|ident| ident.to_string()))
            .expect("Generator Field should have a rename or identifier")
    }
}

trait ToTypescript {
    fn to_typescript(self) -> Result<String, Error>;
}

// TODO: Find how to "flatten" types
pub(crate) fn generate_typescript_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };
    let GeneratorArgs { directory } = match GeneratorArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };
    let derive_input = parse_macro_input!(input as ItemStruct).into();
    let generator_input = match GeneratorInputReceiver::from_derive_input(&derive_input) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };
    let output_dir = absolute(directory).expect("Output directory should be absolute");
    dbg!(&output_dir);
    let path = output_dir.join(generator_input.name() + ".ts");
    dbg!(&path);

    let mut file = match File::create(path) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(Error::custom(e).write_errors())
    };
    let contents = match generator_input.to_typescript() {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()) }
    };

    if let Err(e) = file.write_all(contents.as_bytes()) {
        return TokenStream::from(Error::custom(e).write_errors());
    }

    let result = quote! {
        #derive_input
    };

    result.into()
}