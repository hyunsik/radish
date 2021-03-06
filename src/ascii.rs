//! A collection of ascii representation (u8) utility functions
use std::str::{self, FromStr};

use err::ParseNumErr;

pub trait FromBytes: Sized {
  type Err;
  unsafe fn from_bytes(b: &[u8]) -> Result<Self, Self::Err>;
}

macro_rules! impl_from_bytes {
  ($ty:ident) => {
    impl FromBytes for $ty {
      type Err = ParseNumErr;
      unsafe fn from_bytes(b: &[u8]) -> Result<$ty, ParseNumErr> {
        Ok($ty::from_str(str::from_utf8_unchecked(b))?)
      }
    }
  }
}

impl_from_bytes!(u8);
impl_from_bytes!(i8);
impl_from_bytes!(i16);
impl_from_bytes!(u16);
impl_from_bytes!(i32);
impl_from_bytes!(u32);
impl_from_bytes!(i64);
impl_from_bytes!(u64);
impl_from_bytes!(f32);
impl_from_bytes!(f64);

/// Returns true if this `u8` is an alphabetic code point, and false if not.
#[inline]
pub fn isalpha(c: u8) -> bool {
  match c {
    b'a'...b'z' | b'A'...b'Z' => true,
    _ => false
  }
}

/// Checks if a `char` is a digit in a radix of ten.
#[inline]
pub fn isdigit(c: u8) -> bool {
  b'0' <= c && c <= b'9'
}

/// Checks if a `char` is a digit in a radix of hexadecimal.
#[inline]
pub fn isdigit_hex(c: u8) -> bool {
  match c {
    b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => true,
    _ => false
  }
}

/// Checks if a given character is an alphanumerical character.
#[inline]
pub fn isalnum(c: u8) -> bool {
  isalpha(c) || isdigit(c)
}

/// Checks if a given character is a space character (i.e., ' ', \t, \n, \r).
#[inline]
pub fn isspace(c: u8) -> bool {
  match c {
    b' ' | b'\t' | b'\n' | b'\r' => true,
    _ => false
  }
}

pub unsafe fn strtof<'a>(s: &'a [u8])
    -> Result<(f32, Option<&'a [u8]>), ParseNumErr> {

  let (val, remain) = strtod(s)?;
  if val > ::std::f32::MAX as f64 {
    Err(ParseNumErr::overflow())
  } else {
    Ok((val as f32, remain))
  }
}

/// Convert a string to an integer value and
/// return a remaning part which is not valid in decimal
pub unsafe fn strtod<'a>(s: &'a [u8])
    -> Result<(f64, Option<&'a [u8]>), ParseNumErr> {

  let start_idx = match s.iter().position(|&c| !isspace(c)) {
    Some(idx) => idx,
    None => s.len()
  };

  let end_idx = match s[start_idx..].iter().position(|&c| !(isdigit(c) || c == b'.')) {
    Some(idx) => idx,
    None => s.len()
  };

  let val = f64::from_str(str::from_utf8_unchecked(&s[start_idx..end_idx]))?;

  let remain = if end_idx < s.len() {
    Some(&s[end_idx..])
  } else {
    None
  };

  Ok((val, remain))
}

pub unsafe fn strtoi<'a>(s: &'a [u8])
    -> Result<(i32, Option<&'a [u8]>), ParseNumErr> {

  let (val, remain) = strtol(s)?;
  if val != ((val as i32) as i64) {
    Err(ParseNumErr::overflow())
  } else {
    Ok((val as i32, remain))
  }
}


/// Convert a string to an integer value and
/// return a remaning part which is not valid in decimal
pub unsafe fn strtol<'a>(s: &'a [u8])
  -> Result<(i64, Option<&'a [u8]>), ParseNumErr> {

  let start_idx = match s.iter().position(|&c| !isspace(c)) {
    Some(idx) => idx,
    None => s.len()
  };

  let end_idx = match s[start_idx..].iter().position(|&c| !isdigit(c)) {
    Some(idx) => idx,
    None => s.len()
  };

  let val = i64::from_str(str::from_utf8_unchecked(&s[start_idx..end_idx]))?;

  let remain = if end_idx < s.len() {
    Some(&s[end_idx..])
  } else {
    None
  };

  Ok((val, remain))
}

const DIFF_BETWEEN_LOWER_UPPER: u8 = b'a' - b'A';

#[inline]
pub fn tolowerc(c: u8) -> u8 {  
  if c < b'A' || b'Z' < c {
    c
  } else { 
    c + DIFF_BETWEEN_LOWER_UPPER
  }
}

pub fn tolower(s: &[u8]) -> Vec<u8> {
  s.iter().map(|c| tolowerc(*c)).collect::<Vec<u8>>()
}

#[inline]
pub fn toupperc(c: u8) -> u8 {  
  if c < b'a' || b'z' < c {
    c
  } else { 
    c - DIFF_BETWEEN_LOWER_UPPER
  }
}

pub fn toupper(s: &[u8]) -> Vec<u8> {
  s.iter().map(|c| toupperc(*c)).collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
  use super::{strtod, strtol, strtoi, tolowerc, tolower, toupperc, toupper};

  #[test]
  fn test_strtol() {
    unsafe {
      let (val, remain) = strtol(b"   12345").ok().unwrap();
      assert_eq!(12345i64, val);
      assert!(remain.is_none());

      let (val, remain) = strtol(b"12345").ok().unwrap();
      assert_eq!(12345i64, val);
      assert!(remain.is_none());

      let (val, remain) = strtol(b"12345l").ok().unwrap();
      assert_eq!(12345i64, val);
      assert!(remain.is_some());
      assert_eq!(b"l", remain.unwrap());

      let (val, remain) = strtol(b"12345lll").ok().unwrap();
      assert_eq!(12345i64, val);
      assert!(remain.is_some());
      assert_eq!(b"lll", remain.unwrap());

      let (val, remain) = strtol(b"1").ok().unwrap();
      assert_eq!(1, val);
      assert!(remain.is_none());
    }
  }

  #[test]
  fn test_strtod() {
    unsafe {
      let (val, remain) = strtod(b"   12345.123").ok().unwrap();
      assert_eq!(12345.123f64, val);
      assert!(remain.is_none());

      let (val, remain) = strtod(b"12345.123").ok().unwrap();
      assert_eq!(12345.123f64, val);
      assert!(remain.is_none());

      let (val, remain) = strtod(b"12345.123l").ok().unwrap();
      assert_eq!(12345.123f64, val);
      assert!(remain.is_some());
      assert_eq!(b"l", remain.unwrap());

      let (val, remain) = strtod(b"12345.123lll").ok().unwrap();
      assert_eq!(12345.123f64, val);
      assert!(remain.is_some());
      assert_eq!(b"lll", remain.unwrap());

      let (val, remain) = strtod(b"1.1").ok().unwrap();
      assert_eq!(1.1f64, val);
      assert!(remain.is_none());
    }
  }

  #[test]
  #[should_panic]
  fn test_strtoi_overflow() {
    unsafe { strtoi(b"123456789012345lll").ok().unwrap() };
  }

  #[test]
  fn test_tolowerc() {
    assert_eq!(b'a', tolowerc(b'A'));
    assert_eq!(b'a', tolowerc(b'a'));
    assert_eq!(b'b', tolowerc(b'B'));
  }

  #[test]
  fn test_tolower() {
    assert_eq!(b"abcdefgh", tolower(b"aBcdEfGH").as_slice());
  }

  #[test]
  fn test_toupperc() {
    assert_eq!(b'A', toupperc(b'A'));
    assert_eq!(b'A', toupperc(b'a'));
    assert_eq!(b'B', toupperc(b'b'));
  }

  #[test]
  fn test_toupper() {
    assert_eq!(b"ABCDEFGH", toupper(b"aBcdEfGH").as_slice());
  }
}