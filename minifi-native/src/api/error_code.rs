use std::num::ParseIntError;
use std::str::ParseBoolError;
use minifi_native_sys::{MinifiStatus, MinifiStatus_MINIFI_UNKNOWN_ERROR};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Strum(strum::ParseError),
    Bool(ParseBoolError),
    Int(ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MinifiError {
    UnknownError,
    MissingRequiredProperty(&'static str),
    Parse(ParseError)
}

impl From<strum::ParseError> for MinifiError {
    fn from(err: strum::ParseError) -> Self {
        MinifiError::Parse(ParseError::Strum(err))
    }
}

impl From<ParseBoolError> for MinifiError {
    fn from(err: ParseBoolError) -> Self {
        MinifiError::Parse(ParseError::Bool(err))
    }
}

impl From<ParseIntError> for MinifiError {
    fn from(err: ParseIntError) -> Self {
        MinifiError::Parse(ParseError::Int(err))
    }
}

impl MinifiError {
    pub(crate) fn to_status(&self) -> MinifiStatus{
        MinifiStatus_MINIFI_UNKNOWN_ERROR
    }
}
