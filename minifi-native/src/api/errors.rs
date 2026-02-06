use minifi_native_sys::{MinifiStatus, MinifiStatus_MINIFI_STATUS_UNKNOWN_ERROR};
use std::num::ParseIntError;
use std::str::ParseBoolError;

#[derive(Debug, Clone)]
pub struct SizeParseError(pub byte_unit::ParseError);

impl PartialEq for SizeParseError {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for SizeParseError {}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Strum(strum::ParseError),
    Bool(ParseBoolError),
    Int(ParseIntError),
    Duration(humantime::DurationError),
    Size(SizeParseError),
    Other,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MinifiError {
    UnknownError,
    MissingRequiredProperty(&'static str),
    ControllerServiceError(&'static str),
    InvalidValidator,
    Parse(ParseError),
    ScheduleError(String),
    TriggerError(String),
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

impl From<humantime::DurationError> for MinifiError {
    fn from(err: humantime::DurationError) -> Self {
        MinifiError::Parse(ParseError::Duration(err))
    }
}

impl From<byte_unit::ParseError> for MinifiError {
    fn from(err: byte_unit::ParseError) -> Self {
        MinifiError::Parse(ParseError::Size(SizeParseError(err)))
    }
}

impl MinifiError {
    pub(crate) fn to_status(&self) -> MinifiStatus {
        MinifiStatus_MINIFI_STATUS_UNKNOWN_ERROR
    }
}
