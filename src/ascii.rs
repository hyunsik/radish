//! A collection of ascii representation (u8) utility functions

use std::str::{self, FromStr};

pub trait Ascii: Clone {
  fn is_alpha(self) -> bool;
  fn is_digit(self) -> bool;
  fn is_hex_digit(self) -> bool;
  fn is_space(self) -> bool;
}

impl Ascii for u8 {
  /// Returns true if this `u8` is an alphabetic code point, and false if not.
  #[inline]
  fn is_alpha(self) -> bool {
    match self {
      b'a'...b'z' | b'A'...b'Z' => true,
      _ => false
    }
  }

  /// Checks if a `char` is a digit in a radix of ten.
  #[inline]
  fn is_digit(self) -> bool {
    b'0' <= self && self <= b'9'
  }

  /// Checks if a `char` is a digit in a radix of hexadecimal.
  #[inline]
  fn is_hex_digit(self) -> bool {
    match self {
      b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => true,
      _ => false
    }
  }

  #[inline]
  fn is_space(self) -> bool {
    match self {
      b' ' | b'\t' | b'\n' | b'\r' => true,
      _ => false
    }
  }
}

/// Convert a string to a i64 value and
/// return a remaning part which is not valid in decimal
pub fn strtol<'a>(s: &'a str, start_idx: usize) -> (i64, Option<&'a str>) {
  debug_assert!(s.len() > start_idx, format!(
    "the length of input string must be greater than start_idx, \
    but length = {} and start_idx = {}", s.len(), start_idx));

  let bytes = &(s.as_bytes()[start_idx..]);
  let last_digit_idx = match bytes.iter().position(|&c| !c.is_digit()) {
    Some(idx) => idx,
    None => bytes.len()
  };

  let val = i64::from_str(unsafe {
      str::from_utf8_unchecked(&bytes[..last_digit_idx])
    }).ok().unwrap();

  let remain = if s.len() == last_digit_idx {
    None
  } else {
    Some(unsafe {str::from_utf8_unchecked(&bytes[last_digit_idx..])})
  };

  (val, remain)
}

#[cfg(test)]
mod tests {
  use super::strtol;

  #[test]
  fn test_strtol() {
    let (val, remain) = strtol("12345", 0);
    assert_eq!(12345i64, val);
    assert!(remain.is_none());

    let (val, remain) = strtol("12345l", 0);
    assert_eq!(12345i64, val);
    assert!(remain.is_some());
    assert_eq!("l", remain.unwrap());

    let (val, remain) = strtol("12345lll", 0);
    assert_eq!(12345i64, val);
    assert!(remain.is_some());
    assert_eq!("lll", remain.unwrap());
  }
}