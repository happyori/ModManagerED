use anyhow::anyhow;
use regex::RegexBuilder;
use syn::{GenericArgument, PathArguments, Type};

pub trait ToTypescript {
    type Error;
    fn to_typescript(self) -> Result<String, Self::Error>;
}

impl ToTypescript for Type {
    type Error = anyhow::Error;
    fn to_typescript(self) -> Result<String, Self::Error> {
        let not_implemented_error = "This type is not implemented yet or is incompatible with typescript";
        match self {
            Type::Array(v) => Ok(format!("{}[]", v.elem.to_typescript()?)),
            Type::Ptr(v) => Ok(v.elem.to_typescript()?),
            Type::Reference(v) => Ok(v.elem.to_typescript()?),
            Type::Paren(v) => Ok(v.elem.to_typescript()?),
            Type::Slice(v) => Ok(format!("{}[]", v.elem.to_typescript()?)),
            Type::Tuple(v) => {
                let inner_types = v
                    .elems
                    .into_iter()
                    .map(|p| p.to_typescript())
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");
                Ok(format!("[{inner_types}]"))
            }
            Type::Path(v) => {
                let last_segment = v
                    .path
                    .segments
                    .last()
                    .ok_or(anyhow!(not_implemented_error))?;
                let ident_string = last_segment.ident.to_string();

                let extract_inner = || {
                    match &last_segment.arguments {
                        PathArguments::AngleBracketed(pargs) => {
                            let inner_type = pargs
                                .args
                                .iter()
                                .filter_map(|a| {
                                    let GenericArgument::Type(ty) = a else {
                                        return None;
                                    };
                                    Some(ty)
                                })
                                .last()
                                .ok_or(anyhow!("Vector should have a type"))?;
                            Ok(inner_type.clone())
                        }
                        PathArguments::None => Err(anyhow!("Vector has to have a type")),
                        PathArguments::Parenthesized(_) => Err(anyhow!(not_implemented_error)),
                    }
                };

                // Handle Vectors
                if ident_string == "Vec" && !last_segment.arguments.is_empty() {
                    let inner = extract_inner()?;
                    return Ok(format!("Array<{}>", inner.to_typescript()?));
                }

                //Handle Options
                if ident_string == "Option" && !last_segment.arguments.is_empty() {
                    let inner = extract_inner()?;
                    return Ok(format!("{} | undefined", inner.to_typescript()?));
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

                Ok(ident_string)
            }
            Type::ImplTrait(_) => Err(anyhow!(not_implemented_error)),
            Type::Group(_) => Err(anyhow!(not_implemented_error)),
            Type::Macro(_) => Err(anyhow!(not_implemented_error)),
            Type::Never(_) => Ok("never".to_string()),
            _ => Ok("any".to_string()),
        }
    }
}