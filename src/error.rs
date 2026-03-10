use std::error::Error;
use std::ffi::NulError;
use std::fmt::{Display, Formatter};
use crate::error::XlrnError::StdError;

#[derive(Debug)]
pub enum XlrnError {
    WorkBookNull,
    WorkSheetNull,
    FileNotFound,
    StdError(Box<dyn Error + Sync + Send>),
}

impl From<NulError> for XlrnError {
    fn from(value: NulError) -> Self {
        StdError(Box::new(value))
    }
}

impl Display for XlrnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XlrnError::WorkBookNull => write!(f, "work book null"),
            XlrnError::WorkSheetNull => write!(f, "work sheet null"),
            XlrnError::FileNotFound => write!(f, "file not found"),
            StdError(err) => write!(f, "{:?}", err),
        }
    }
}

pub type XlrnResult<T> = Result<T, XlrnError>;