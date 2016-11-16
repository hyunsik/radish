use std::convert::From;
use std::num::{ParseIntError, ParseFloatError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseNumError { kind: ParseNumErrorKind }

impl ParseNumError {
  pub fn overflow() -> ParseNumError {
    ParseNumError {
      kind: ParseNumErrorKind::Overflow
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseNumErrorKind {
  Empty,
  InvalidDigit,
  Overflow,
  Underflow,
  BadFormat(String)
}

impl From<ParseIntError> for ParseNumError {
  fn from(e: ParseIntError) -> Self {
    ParseNumError {
      kind: ParseNumErrorKind::BadFormat(format!("{}", e))
    }
  }
}

impl From<ParseFloatError> for ParseNumError {
  fn from(e: ParseFloatError) -> Self {
    ParseNumError {
      kind: ParseNumErrorKind::BadFormat(format!("{}", e))
    }
  }
}