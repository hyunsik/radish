//! A collection of ascii representation (u8) utility functions

use std::str::{self, FromStr};

pub trait Ascii: Clone {
  fn is_alpha(self) -> bool;
  fn is_digit(self) -> bool;
  fn is_hex_digit(self) -> bool;
  fn is_alnum(self) -> bool;
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
  fn is_alnum(self) -> bool {
    self.is_alpha() || self.is_digit()
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
pub fn strtoi<'a>(s: &'a [u8], start_idx: usize) -> (i32, Option<&'a [u8]>) {
  let (val, remain) = strtol(s, start_idx);
  if val != ((val as i32) as i64) {
    panic!(format!("integer overflow in strtoi: {}",
      unsafe { str::from_utf8_unchecked(s) }));
  }
  return (val as i32, remain);
}

/// Convert a string to a i64 value and
/// return a remaning part which is not valid in decimal
pub fn strtol<'a>(s: &'a [u8], start_idx: usize) -> (i64, Option<&'a [u8]>) {
  debug_assert!(s.len() > start_idx, format!(
    "the length of input string must be greater than start_idx, \
    but length = {} and start_idx = {}", s.len(), start_idx));

  let bytes = &s[start_idx..];
  let last_digit_idx = match bytes.iter().position(|&c| !c.is_digit()) {
    Some(idx) => idx,
    None => bytes.len()
  };

  let val = i64::from_str(unsafe {
      str::from_utf8_unchecked(&bytes[..last_digit_idx])
    }).ok().unwrap();

  let remain = if bytes.len() == last_digit_idx {
    None
  } else {
    Some(&bytes[last_digit_idx..])
  };

  (val, remain)
}

#[cfg(test)]
mod tests {
  use super::{strtoi, strtol};

  #[test]
  fn test_strtol() {
    let (val, remain) = strtol(b"12345", 0);
    assert_eq!(12345i64, val);
    assert!(remain.is_none());

    let (val, remain) = strtol(b"12345l", 0);
    assert_eq!(12345i64, val);
    assert!(remain.is_some());
    assert_eq!(b"l", remain.unwrap());

    let (val, remain) = strtol(b"12345lll", 0);
    assert_eq!(12345i64, val);
    assert!(remain.is_some());
    assert_eq!(b"lll", remain.unwrap());

    let (val, remain) = strtol(b"+1", 1);
    assert_eq!(1, val);
    assert!(remain.is_none());
  }

  #[test]
  #[should_panic]
  fn test_strtoi_overflow() {
    strtoi(b"123456789012345lll", 0);
  }
}