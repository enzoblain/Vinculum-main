use crate::codegen::types::{FfiLangType, Struct};

pub(super) fn generate_struct<T>(s: Struct<T>, file: &mut String)
where
    T: FfiLangType,
{
    let documentation = s
        .documentation
        .iter()
        .map(|doc| format!("/// {}", doc))
        .collect::<Vec<_>>()
        .join("\n");

    let derives_attr = if s.derives.is_empty() {
        String::new()
    } else {
        let derives_list = s
            .derives
            .iter()
            .map(|d| d.as_ref())
            .collect::<Vec<_>>()
            .join(", ");
        format!("#[derive({})]\n", derives_list)
    };

    let fields = s
        .fields
        .iter()
        .map(|field| format!("    pub {}: {}", field.name, field.r#type.rust_type_name()))
        .collect::<Vec<_>>()
        .join(",\n");

    let content = if s.fields.is_empty() {
        format!(
            "{}{}#[allow(dead_code)]\npub struct {} {{}}\n",
            if documentation.is_empty() {
                String::new()
            } else {
                format!("{}\n", documentation)
            },
            derives_attr,
            s.name
        )
    } else {
        format!(
            "{}{}#[allow(dead_code)]\npub struct {} {{\n{},\n}}\n",
            if documentation.is_empty() {
                String::new()
            } else {
                format!("{}\n", documentation)
            },
            derives_attr,
            s.name,
            fields
        )
    };

    file.push_str(content.as_str());
}
