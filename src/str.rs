pub fn join<S: AsRef<str>, T: IntoIterator<Item=S>>(strs: T, delimiter: &str) -> String {
  let mut buf = String::new();
  let mut first = true;

  for s in strs.into_iter() {
    
    if first {
      first = false;
    } else {
      buf.push_str(delimiter);
    }

    buf.push_str(s.as_ref());
  }

  buf
}

pub fn join_with_map<'a, S, T: IntoIterator<Item=S>, F: Fn(S) -> &'a str>(strs: T, delimiter: &str, f: F) -> String {

  let mut buf = String::new();
  let mut first = true;

  for s in strs.into_iter() {
    
    if first {
      first = false;
    } else {
      buf.push_str(delimiter);
    }

    buf.push_str(f(s));
  }

  buf
}

#[cfg(test)]
mod tests {
  use super::{join, join_with_map};

  #[test]
  fn test_join() {
    let str_vec = vec!["a", "b", "c"];
    let str_arr = ["a", "b", "c"];
    assert_eq!("a,b,c".to_string(), join(&str_vec, ","));
    assert_eq!("a,b,c".to_string(), join(&str_arr, ","));

    assert_eq!("a,b,c".to_string(), join_with_map(&str_vec, ",", |x| x));
    assert_eq!("a,b,c".to_string(), join_with_map(&str_arr, ",", |x| x));
  }
}
