use proc_macro::TokenStream;
use std::fs::File;
use std::io::Write;
use std::path::{absolute, PathBuf};

use darling::{ast, Error, FromDeriveInput, FromField};
use quote::quote;
use syn::{DeriveInput, parse_macro_input, Path, Type, TypePath};

use utility::ToTypescript;

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
    directory: PathBuf,
}

impl ToTypescript for GeneratorField {
    type Error = anyhow::Error;
    fn to_typescript(self) -> Result<String, Self::Error> {
        let name = self.name();
        let Self {
            ty,
            ignored,
            any,
            typed_as,
            ..
        } = self;
        if ignored {
            return Ok("".into());
        }
        if any {
            return Ok(format!("{name}: any"));
        }
        if let Some(typ) = typed_as {
            let syn_type = TypePath {
                path: typ,
                qself: None,
            };
            return Ok(format!("{name}: {}", Type::Path(syn_type).to_typescript()?));
        }

        Ok(format!("{name}: {}", ty.to_typescript()?))
    }
}

impl ToTypescript for GeneratorInputReceiver {
    type Error = anyhow::Error;
    fn to_typescript(self) -> Result<String, Self::Error> {
        let Self {
            ident,
            data,
            rename,
            ..
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

        let result = format!(
            "{import_lines}\nexport type {name} = {{\n    {};\n}}\n",
            ts_fields.trim_end()
        );

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

pub(crate) fn generate_typescript_impl(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let generator_input = match GeneratorInputReceiver::from_derive_input(&derive_input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let output_dir =
        absolute(generator_input.directory.clone()).expect("Output directory should be absolute");
    let path = output_dir.join(generator_input.name() + ".ts");

    let mut file = match File::create(path) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(Error::custom(e).write_errors()),
    };
    let contents = match generator_input.to_typescript() {
        Ok(v) => v,
        Err(e) => return TokenStream::from(Error::custom(e).write_errors()),
    };

    if let Err(e) = file.write_all(contents.as_bytes()) {
        return TokenStream::from(Error::custom(e).write_errors());
    }

    let result = quote! {};
    result.into()
}