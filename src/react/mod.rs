//!
//! A user can send any message to EventBus.
//!

pub mod actor;
pub mod dispatcher;

use std::sync::{Arc};

use env_logger;
use rustc_serialize::Decodable;

use self::dispatcher::{Dispatcher, AsyncDispatcher};

pub use self::actor::Actor;

pub trait MsgTrait: 'static + Sync + Send + Decodable {}
pub trait Error: 'static + Sized + Sync + Send {}

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