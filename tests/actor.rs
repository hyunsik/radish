extern crate radish;
extern crate env_logger;
#[macro_use] extern crate log;
extern crate rustc_serialize;
  
use std::thread;
use std::time::Duration;

use radish::react::{Actor, MsgTrait, Error};

  // #[derive(RustcDecodable, RustcEncodable)]
  // pub enum Message {
  //   Ask(String),
  //   Others
  // }

  // impl Message {
  //   pub fn ask(message: &str) -> Message {
  //     Message::Ask(message.to_owned())
  //   }
  // }

  // impl MsgTrait for Message {}
  // unsafe impl Send for Message {}
  // unsafe impl Sync for Message {}

  // pub struct TestActor;

  // pub enum ActorErr {
  //   Err(String)
  // }
  // impl Error for ActorErr {}
  // unsafe impl Send for ActorErr {}
  // unsafe impl Sync for ActorErr {}

  // impl Actor<Message, ActorErr> for TestActor {
  //   fn on_receive(&mut self, m: &Message) -> Result<(), ActorErr> {
  //     match *m {
  //       Message::Ask(ref s) => {
  //         debug!(">>> received: {}", s);
  //         Ok(())
  //       }
  //       _ => panic!("unknown message")
  //     }
  //   }
  // }

  #[test]
  fn test() {
    env_logger::init().unwrap();

    // let actors: Vec<Box<Actor<Message, ActorErr>>> = vec![Box::new(TestActor)];
    // let mut dispatcher = Dispatcher::new(actors);

    // let time = Duration::from_secs(5);
    // dispatcher.send(Message::Ask("abc".to_owned()));
    // thread::sleep(time);

    // debug!("after sleep");
    // dispatcher.stop();
    // debug!("after stop");
    // dispatcher.join().ok().unwrap();
  }