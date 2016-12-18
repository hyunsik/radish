//!
//! A user can send any message to EventBus.
//!

pub mod actor;
pub mod dispatcher;

use std::sync::{Arc};

use env_logger;
use rustc_serialize::Decodable;

use self::dispatcher::{Dispatcher, AsyncDispatcher};

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

#[cfg(test)]
mod tests {
  use std::thread;
  use std::time::Duration;

  use env_logger;
  use super::*;

  #[derive(RustcDecodable, RustcEncodable)]
  pub enum Message {
    Ask(String),
    Others
  }

  impl Message {
    pub fn ask(message: &str) -> Message {
      Message::Ask(message.to_owned())
    }
  }

  impl MsgTrait for Message {}
  unsafe impl Send for Message {}
  unsafe impl Sync for Message {}

  pub struct TestActor;

  pub enum ActorErr {
    Err(String)
  }
  impl Error for ActorErr {}
  unsafe impl Send for ActorErr {}
  unsafe impl Sync for ActorErr {}

  impl Actor<Message, ActorErr> for TestActor {
    fn on_receive(&mut self, m: &Message) -> Result<(), ActorErr> {
      match *m {
        Message::Ask(ref s) => {
          debug!(">>> received: {}", s);
          Ok(())
        }
        _ => panic!("unknown message")
      }
    }
  }

  #[test]
  fn test() {
    env_logger::init().unwrap();

    let actors: Vec<Box<Actor<Message, ActorErr>>> = vec![Box::new(TestActor)];
    let mut dispatcher = Dispatcher::new(actors);

    let time = Duration::from_secs(5);
    dispatcher.send(Message::Ask("abc".to_owned()));
    thread::sleep(time);

    debug!("after sleep");
    dispatcher.stop();
    debug!("after stop");
    dispatcher.join().ok().unwrap();
  }
}