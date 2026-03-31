use std::slice;

use crate::ffi::{call_haskell_function, free_haskell_buffer};

include!(concat!(env!("OUT_DIR"), "/generated_functions.rs"));

enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
}

impl Value {
    fn into_int(self) -> i64 {
        match self {
            Value::Int(value) => value,
            _ => panic!("Expected Value::Int"),
        }
    }

    fn into_float(self) -> f64 {
        match self {
            Value::Float(value) => value,
            _ => panic!("Expected Value::Float"),
        }
    }

    fn into_bool(self) -> bool {
        match self {
            Value::Bool(value) => value,
            _ => panic!("Expected Value::Bool"),
        }
    }

    fn into_string(self) -> String {
        match self {
            Value::String(value) => value,
            _ => panic!("Expected Value::String"),
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        match self {
            Value::Bytes(value) => value,
            _ => panic!("Expected Value::Bytes"),
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        match self {
            Value::Int(x) => {
                buf.push(0);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Float(x) => {
                buf.push(1);
                buf.extend_from_slice(&x.to_le_bytes());
            }
            Value::Bool(b) => {
                buf.push(2);
                buf.push(*b as u8);
            }
            Value::String(s) => {
                buf.push(3);
                let bytes = s.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
            Value::Bytes(b) => {
                buf.push(4);
                buf.extend_from_slice(&(b.len() as u64).to_le_bytes());
                buf.extend_from_slice(b);
            }
        }

        buf
    }

    fn encode_slice(args: &[Value]) -> Vec<u8> {
        let mut buf = Vec::new();

        for arg in args {
            buf.extend_from_slice(&arg.to_bytes());
        }

        buf
    }

    fn from_bytes(bytes: &[u8]) -> Value {
        let tag = bytes[0];

        match tag {
            0 => {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Value::Int(i64::from_le_bytes(arr))
            }
            1 => {
                let mut arr = [0u8; 8];
                arr.copy_from_slice(&bytes[1..9]);
                Value::Float(f64::from_le_bytes(arr))
            }
            2 => Value::Bool(bytes[1] != 0),
            3 => {
                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&bytes[1..9]);
                let len = u64::from_le_bytes(len_bytes) as usize;

                let s = String::from_utf8(bytes[9..9 + len].to_vec()).expect("Invalid UTF-8");
                Value::String(s)
            }
            4 => {
                let mut len_bytes = [0u8; 8];
                len_bytes.copy_from_slice(&bytes[1..9]);
                let len = u64::from_le_bytes(len_bytes) as usize;

                Value::Bytes(bytes[9..9 + len].to_vec())
            }
            _ => panic!("Invalid type tag"),
        }
    }
}

pub(super) fn call_haskell_typed(name: &str, args: &[Value]) -> Value {
    let input = Value::encode_slice(args);

    let mut out_ptr: *mut u8 = std::ptr::null_mut();
    let mut out_len: usize = 0;

    unsafe {
        call_haskell_function(
            name.as_ptr(),
            name.len(),
            input.as_ptr(),
            input.len(),
            &mut out_ptr as *mut *mut u8,
            &mut out_len as *mut usize,
        );
    }

    if out_ptr.is_null() {
        panic!("Haskell returned a null buffer");
    }

    let bytes = unsafe { slice::from_raw_parts(out_ptr, out_len) };
    let result = Value::from_bytes(bytes);

    unsafe {
        free_haskell_buffer(out_ptr);
    }

    result
}
