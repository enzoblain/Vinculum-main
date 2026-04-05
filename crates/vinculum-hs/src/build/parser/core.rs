use std::fs;
use std::path::Path;

use super::args::contain_all_args;
use super::comments::is_only_comment;
use super::errors::ParseError;
use super::functions::{is_signature, push_function, Function, FunctionBuffer};

pub(crate) fn extract_functions(path: impl AsRef<Path>) -> Result<Vec<Function>, ParseError> {
    let path = path.as_ref();
    let content = fs::read_to_string(path).map_err(|e| ParseError::ReadFile {
        path: path.to_path_buf(),
        source: e,
    })?;

    let mut buf = FunctionBuffer::default();
    let mut checkpoint_line: Option<usize> = None; // To remember the next function description
    let mut functions = Vec::new();

    let lines: Vec<&str> = content.lines().collect();
    let len = lines.len();
    let mut i = 0;

    while i < len {
        let raw_line = lines[i];
        let line = raw_line.trim();

        if line.is_empty() {
            // The comments used for the function description should be stacked
            // So, when we find an empty_line, the checkpoint is forgotten
            checkpoint_line = None;

            // Same logic for the actual description
            if buf.signature.is_empty() {
                buf.description.clear();
            }

            i += 1;
            continue;
        }

        if let Some(comment) = is_only_comment(raw_line) {
            if buf.signature.is_empty() {
                buf.description.push(comment.to_string());
            } else {
                checkpoint_line.get_or_insert(i);
            }

            i += 1;
            continue;
        }

        let code_line = raw_line
            .split_once("--")
            .map(|(before, _)| before)
            .unwrap_or(raw_line)
            .trim();

        if let Some((signature, is_new)) = is_signature(code_line) {
            if is_new && !buf.signature.is_empty() {
                push_function(&mut functions, &mut buf)?;

                // Get back to the function description beginning
                if let Some(checkpoint) = checkpoint_line.take() {
                    i = checkpoint;
                    continue;
                }
            }

            if !is_new {
                buf.signature.push(' ');
            }

            buf.signature.push_str(signature);
            i += 1;
            continue;
        }

        if !buf.signature.is_empty()
            && let Some(args) = contain_all_args(code_line)
        {
            buf.args = args.iter().map(|s| s.to_string()).collect();

            push_function(&mut functions, &mut buf)?;
            i += 1;
            continue;
        }

        i += 1;
    }

    // Don't forget last function if no explicit args have been found
    if !buf.signature.is_empty() {
        push_function(&mut functions, &mut buf)?;
    }

    Ok(functions)
}
