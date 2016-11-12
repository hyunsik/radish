//! A collection of byte (u8) utility functions

pub trait Ascii: Clone {
  fn is_alpha(self) -> bool;
  fn is_digit(self) -> bool;
  fn is_hex_digit(self) -> bool;
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
    match self {
      b'0'...b'9' => true,
      _ => false
    }
  }

  /// Checks if a `char` is a digit in a radix of hexadecimal.
  #[inline]
  fn is_hex_digit(self) -> bool {
    match self {
      b'0'...b'9' | b'a'...b'f' | b'A'...b'F' => true,
      _ => false
    }
  }
}