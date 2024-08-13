#[derive(Debug, thiserror::Error)]
pub enum CaciError {
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
}

pub type CaciResult<T> = anyhow::Result<T, CaciError>;