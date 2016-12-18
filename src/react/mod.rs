//!
//! A user can send any message to EventBus.
//!

pub mod actor;
pub mod dispatcher;

use std::sync::{Arc};

use env_logger;
use rustc_serialize::Decodable;

pub use self::dispatcher::Dispatcher;
pub use self::actor::Actor;

use self::dispatcher::AsyncDispatcher;

pub trait MsgTrait: 'static + Sync + Send + Decodable {}
pub trait Error: 'static + Sized + Sync + Send {}

pub type Predicate<T> = Fn(&T) -> bool;

pub struct ActorSystem<M: MsgTrait, E: Error> {
  dispatcher: Arc<Box<Dispatcher<M, E>>>  
}

impl<M: MsgTrait, E: Error> ActorSystem<M, E> {
  pub fn new(name: &str) -> ActorSystem<M, E> {
    ActorSystem {
      dispatcher: Arc::new(Box::new(AsyncDispatcher::new()) as Box<Dispatcher<M, E>>)
    } 
  }

  pub fn dispatcher(&self) -> Arc<Box<Dispatcher<M, E>>> {
    self.dispatcher.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(RustcDecodable, RustcEncodable)]
  pub enum Msg {
    Ask
  }

  impl MsgTrait for Msg {}
  unsafe impl Send for Msg {}
  unsafe impl Sync for Msg {}
  
  #[derive(RustcDecodable, RustcEncodable)]
  pub enum Err {    
    Fatal
  } 

  impl Error for Err {}
  unsafe impl Send for Err {}
  unsafe impl Sync for Err {}

  #[test]
  fn test() {
    let system: ActorSystem<Msg, Err> = ActorSystem::new("test");
    let dispatcher = system.dispatcher();
  }
}