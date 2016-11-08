#![feature(libc)]
extern crate libc;

pub mod ffi;
pub mod str;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
