use super::types::Function;

pub(crate) fn validate_functions(functions: &[Function]) {
    if functions.is_empty() {
        panic!("Configuration error: no functions defined");
    }

    for func in functions {
        if func.name.trim().is_empty() {
            panic!("Configuration error: function name cannot be empty");
        }

        for arg in &func.args {
            if arg.name.trim().is_empty() {
                panic!("Configuration error: argument name cannot be empty");
            }
        }
    }
}
