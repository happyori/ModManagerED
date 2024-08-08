#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use macros::{GenerateTypescript};
    #[derive(GenerateTypescript)]
    #[gen(directory = "macros/tests/generated")]
    struct Custom {
        yes: String,
        no: bool,
    }
    #[derive(GenerateTypescript)]
    #[gen(directory = "macros/tests/generated")]
    struct Data {
        number: f32,
        boolean: bool,
        string: String,
        cstr: &'static str,
        vec: Vec<String>,
        array: [i32; 4],
        tuple: (i32, String),
        #[gen(import)]
        custom_type: Custom,
        #[gen(ignored)]
        ignored: Custom,
        #[gen(any)]
        convoluted: convoluted::path::inhere::Foobaz,
        #[gen(typed_as = String)]
        retyped: i32,
    }

    mod convoluted {
        pub mod path {
            pub mod inhere {
                pub struct Foobaz;
            }
        }
    }
}