use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use crossbeam::sync::MsQueue;

pub struct EventDispatcher<M: 'static + Sync + Send, E: 'static + Sized + Sync + Send> {
  actors: Arc<Mutex<Vec<Box<Actor<M, E>>>>>,
  queue: Arc<MsQueue<M>>,
  stopped: Arc<Mutex<bool>>,
  thread: JoinHandle<Result<(), E>>,
}

unsafe impl<M: 'static + Sync + Send, E: 'static + Sized + Sync + Send> Sync for EventDispatcher<M, E> {}
unsafe impl<M: 'static + Sync + Send, E: 'static + Sized + Sync + Send> Send for EventDispatcher<M, E> {}

impl<M: Sync + Send, E: Sized + Sync + Send> EventDispatcher<M, E> {

  pub fn new(actors: Vec<Box<Actor<M,E>>>) -> EventDispatcher<M, E> {
    let actors = Arc::new(Mutex::new(actors));
    let queue = Arc::new(MsQueue::new());
    let stopped = Arc::new(Mutex::new(false));
    let sleep_time = Duration::from_millis(50);

    EventDispatcher {
      actors: actors.clone(),
      queue: queue.clone(),
      stopped: stopped.clone(),
      thread: run(stopped, queue, actors)
    }
  }

  pub fn stop(&mut self) {
    debug!("stop enter");
    let mut stopped = self.stopped.lock().unwrap();
    *stopped = true;
    debug!("stop leave");
  }

  pub fn join(self) -> Result<(), E> {
    self.thread.join().unwrap()
  }

  pub fn send(&self, m: M) {
    self.queue.push(m);
  }
}

pub fn run<M, E>(stop: Arc<Mutex<bool>>, queue: Arc<MsQueue<M>>,
    actors: Arc<Mutex<Vec<Box<Actor<M, E>>>>>) -> JoinHandle<Result<(), E>>
    where M: 'static + Send + Sync, E: 'static + Sync + Send {

  debug!("enter run");
  let sleep_time = Duration::from_millis(500);
  thread::spawn(move || -> Result<(), E> {
     loop {
       debug!("loop start");
        if let Some(m) = queue.try_pop() {
          debug!("message exists");
          for actor in actors.lock().unwrap().iter_mut() {
            match actor.on_receive(&m) {
              Err(e) => return Err(e),
              _ => {}
            }
          }
       } else {
         debug!("sleeped");
         thread::sleep(sleep_time);
        }

        if *stop.lock().unwrap() == true {
          break;
        }
      }
      debug!("escaped from loop");
      Ok(())
  })
}

pub trait Actor<M: Sync + Send, E: Sized + Send + Sync>: Send + Sync {
  fn on_receive(&mut self, m: &M) -> Result<(), E>;
}

pub enum Message {
  Ask(String),
  Others
}

impl Message {
  pub fn ask(message: &str) -> Message {
    Message::Ask(message.to_owned())
  }
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

pub struct TestActor;

pub enum ActorErr {
  Err(String)
}

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

// impl<Message, ActorErr> Actor<Message, ActorErr> for TestActor {
//   fn on_receive(&mut self, m: &String) -> Result<(), ActorErr> {
//     Ok(())
//   }
// }

#[cfg(test)]
mod tests {
  use std::thread;
  use std::time::Duration;

  use env_logger;
  use super::*;

  #[test]
  fn test() {
    env_logger::init().unwrap();

    let actors: Vec<Box<Actor<Message, ActorErr>>> = vec![Box::new(TestActor)];
    let mut dispatcher = EventDispatcher::new(actors);

    let time = Duration::from_secs(5);
    dispatcher.send(Message::Ask("abc".to_owned()));
    thread::sleep(time);

    debug!("after sleep");
    dispatcher.stop();
    debug!("after stop");
    dispatcher.join();
  }
}