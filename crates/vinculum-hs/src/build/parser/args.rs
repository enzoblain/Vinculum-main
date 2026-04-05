use super::types::HaskellType;
use super::utils::is_rust_keyword;

pub(crate) struct Arg {
    pub(crate) name: String,
    pub(crate) r#type: HaskellType,
}

impl Arg {
    pub(crate) fn new(name: String, r#type: HaskellType) -> Self {
        Self { name, r#type }
    }
}

pub(crate) fn contain_all_args(code_line: &str) -> Option<Vec<&str>> {
    if code_line.contains('_') {
        return None;
    }

    let definition_part = code_line
        .split_once('=')
        .map(|(before, _)| before)
        .unwrap_or(code_line);

    let raw_args = definition_part
        .split_once(' ')
        .map(|(_, after)| after)
        .unwrap_or("")
        .trim();

    if raw_args.is_empty() {
        return None;
    }

    raw_args
        .split_whitespace()
        .map(|arg| is_valid_variable_name(arg).then_some(arg))
        .collect()
}

#[inline]
pub(crate) fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    if is_rust_keyword(name) {
        return false;
    }

    let mut bytes = name.as_bytes().iter();

    let first = match bytes.next() {
        Some(&c) => c,
        None => return false,
    };

    if !first.is_ascii_lowercase() {
        return false;
    }

    bytes.all(|&c| c.is_ascii_alphanumeric() || c == b'_' || c == b'\'')
}
