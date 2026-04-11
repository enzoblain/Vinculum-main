use super::traits::FfiLangType;
use super::{Function, Struct};

pub struct File<T>
where
    T: FfiLangType,
{
    pub(crate) name: String,
    pub(crate) structs: Vec<Struct<T>>,
    pub(crate) functions: Vec<Function<T>>,
}
