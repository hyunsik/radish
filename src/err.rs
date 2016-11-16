//! Errors
use std::convert::From;
use std::fmt::{self, Display};
use std::num::{ParseIntError, ParseFloatError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseNumErr { kind: ParseNumErrKind }

impl ParseNumErr {
  pub fn overflow() -> ParseNumErr {
    ParseNumErr {
      kind: ParseNumErrKind::Overflow
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseNumErrKind {
  Empty,
  InvalidDigit,
  Overflow,
  Underflow,
  BadFormat(String)
}

impl Display for ParseNumErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.kind {
      ParseNumErrKind::Empty => write!(f, "cannot parse integer from empty string"),
      ParseNumErrKind::InvalidDigit => write!(f, "invalid digit found in string"),
      ParseNumErrKind::Overflow => write!(f, "number too large to fit in target type"),
      ParseNumErrKind::Underflow => write!(f, "number too small to fit in target type"),
      ParseNumErrKind::BadFormat(ref s) => s.fmt(f)
    }
  }
}

impl From<ParseIntError> for ParseNumErr {
  fn from(e: ParseIntError) -> Self {
    ParseNumErr {
      kind: ParseNumErrKind::BadFormat(format!("{}", e))
    }
  }
}

impl From<ParseFloatError> for ParseNumErr {
  fn from(e: ParseFloatError) -> Self {
    ParseNumErr {
      kind: ParseNumErrKind::BadFormat(format!("{}", e))
    }
  }
}