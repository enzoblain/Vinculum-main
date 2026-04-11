use crate::codegen::rust::utils::to_snake_case;
use crate::codegen::types::{FfiLangType, Function};

pub(super) fn generate_function<T>(function: Function<T>, file: &mut String, module: &str)
where
    T: FfiLangType,
{
    let generics = function.generic_resolver.all();

    let type_param = if generics.is_empty() {
        "()".to_string()
    } else if generics.len() == 1 {
        generics[0].clone()
    } else {
        format!("({})", generics.join(", "))
    };

    let signature_args = function
        .args
        .iter()
        .map(|arg| format!("{}: {}", arg.name, arg.r#type.rust_type_name()))
        .collect::<Vec<String>>()
        .join(", ");

    let ffi_arg_exprs = function
        .args
        .iter()
        .map(|arg| arg.r#type.rust_value_expr(&arg.name, type_param.as_str()))
        .collect::<Vec<_>>()
        .join(", ");

    let generics_params = if generics.is_empty() {
        "".to_string()
    } else {
        let bounds = generics
            .iter()
            .map(|g| format!("{g}: Any + Clone + 'static + AcceptedTypes"))
            .collect::<Vec<_>>()
            .join(", ");

        format!("<{}>", bounds)
    };

    let qualified_name = format!("{}_{}", to_snake_case(module), function.name);

    let documentation = function
        .documentation
        .iter()
        .map(|doc| format!("/// {}", doc))
        .collect::<Vec<_>>()
        .join("\n");

    let derives_attr = if function.derives.is_empty() {
        String::new()
    } else {
        let derives_list = function
            .derives
            .iter()
            .map(|d| d.as_ref())
            .collect::<Vec<_>>()
            .join(", ");
        format!("#[derive({})]\n", derives_list)
    };

    let content = format!(
        "{}{}#[allow(non_snake_case, dead_code)]
pub fn {name}{generics}({args_sig}) -> {return_type} {{
    call_haskell_typed(\"{qualified_name}\", &[{args_values}])
        .{conversion}.expect(\"internal FFI type error: invalid return type\")
}}",
        if documentation.is_empty() {
            String::new()
        } else {
            format!("{}\n", documentation)
        },
        derives_attr,
        name = function.name,
        generics = generics_params,
        args_sig = signature_args,
        return_type = function.return_type.rust_type_name(),
        qualified_name = qualified_name,
        args_values = ffi_arg_exprs,
        conversion = function.return_type.rust_return_conversion(),
    );

    file.push_str(content.as_str());
}
