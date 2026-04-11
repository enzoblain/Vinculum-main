use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid argument name: {0}")]
pub struct InvalidArgumentName(pub(crate) String);
