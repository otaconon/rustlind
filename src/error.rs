use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrandError {
    InvalidStrand(Vec<u8>),
}

impl fmt::Display for StrandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrand(seq) => write!(f, "Invalid strand (neither DNA or RNA): '{:?}'", seq),
        }
    }
}

impl std::error::Error for StrandError {}