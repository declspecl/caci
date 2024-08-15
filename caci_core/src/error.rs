#[derive(Debug, thiserror::Error)]
pub enum CaciError {
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    #[error("TOMLDeserializationError: {0}")]
    TOMLDeserializationError(#[from] toml_edit::de::Error),

    #[error("TOMLSerializationError: {0}")]
    TOMLSerializationError(#[from] toml_edit::ser::Error)
}

pub type CaciResult<T> = anyhow::Result<T, CaciError>;
