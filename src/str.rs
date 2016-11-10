use std::fmt::Display;

pub trait Concatable {
  fn concat2(&self) -> String;
  fn join2(&self, delim: &str) -> String;
}

impl<V: Display> Concatable for [V] {
  fn concat2(&self) -> String {
    let mut buf = String::new();
    for x in self.iter() {
      buf.push_str(&format!("{}", x));
    }
    buf
  }

  fn join2(&self, delim: &str) -> String {
    let mut buf = String::new();
    let mut first = true;

    for s in self.iter() {
      if first {
        first = false;
      } else {
        buf.push_str(delim);
      }

      buf.push_str(&format!("{}", s));
    }

    buf
  }
}

pub trait ConcatableWithMap<V, F: Fn(&V) -> String> {
  fn concat_with_map(&self, f: F) -> String;
  fn join_with_map(&self, delim: &str, f: F) -> String;
}

impl<V, F: Fn(&V) -> String> ConcatableWithMap<V, F> for [V] {

  fn concat_with_map(&self, f: F) -> String {
     let mut buf = String::new();
     for v in self.iter() {
       buf.push_str(&f(v));
     }
     buf
  }

  fn join_with_map(&self, delim: &str, f: F) -> String {
    let mut buf = String::new();
    let mut first = true;

    for v in self.iter() {
      if first {
        first = false;
      } else {
        buf.push_str(delim);
      }

      buf.push_str(&f(v));
    }

    buf
  }
}

pub fn join<S, T>(strs: T, delimiter: &str) -> String
    where S: Display,
          T: IntoIterator<Item=S> {

  let mut buf = String::new();
  let mut first = true;

  for s in strs.into_iter() {

    if first {
      first = false;
    } else {
      buf.push_str(delimiter);
    }

    buf.push_str(&format!("{}", s));
  }

  buf
}

pub fn join_with_map<'a, S, T, F>(strs: T, delimiter: &str, f: F) -> String
    where T: IntoIterator<Item=S>,
          F: Fn(S) -> String {

  let mut buf = String::new();
  let mut first = true;

  for s in strs.into_iter() {

    if first {
      first = false;
    } else {
      buf.push_str(delimiter);
    }

    buf.push_str(&f(s));
  }

  buf
}

#[cfg(test)]
mod tests {
  use std::fmt;
  use super::{Concatable, ConcatableWithMap};
  use super::{join, join_with_map};

  pub struct Test {
    x: i32
  }

  impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "<{}>", self.x)
    }
  }

#[test]
  fn test_concatable() {
    let ts = vec![
      Test {x: 1}, Test {x: 2}
    ];
    assert_eq!("<1><2>".to_string(), ts.concat2());
    assert_eq!("<1>=<2>".to_string(), ts.join2("="));

    let str_vec = vec!["a", "b", "c"];
    let str_arr = ["a", "b", "c"];
    assert_eq!("abc".to_string(), str_vec.concat2());
    assert_eq!("abc".to_string(), str_arr.concat2());
  }  

  #[test]
  fn test_concatable_with_map() {
    let str_vec = vec!["a", "b", "c"];
    let str_arr = ["a", "b", "c"];

    assert_eq!("abc".to_string(),
      str_vec.concat_with_map(|x| format!("{}", x)));
    assert_eq!("abc".to_string(),
      str_arr.concat_with_map(|x| format!("{}", x)));

    assert_eq!("a,b,c".to_string(),
      str_vec.join_with_map(",", |x| format!("{}", x)));
    assert_eq!("a,b,c".to_string(),
      str_arr.join_with_map(",", |x| format!("{}", x)));
  }

  #[test]
  fn test_join() {
    let str_vec = vec!["a", "b", "c"];
    let str_arr = ["a", "b", "c"];

    assert_eq!("a,b,c".to_string(), concat(&str_vec, ","));
    assert_eq!("a,b,c".to_string(), concat(&str_arr, ","));

    assert_eq!("a,b,c".to_string(), join(&str_vec, ","));
    assert_eq!("a,b,c".to_string(), join(&str_arr, ","));
  }
}
