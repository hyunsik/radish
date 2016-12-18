use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use crossbeam::sync::MsQueue;

use super::{MsgTrait, Error};
use super::actor::{ActorUri, Actor};

pub struct MessageFrame<M: MsgTrait> {  
  to: ActorUri,
  msg: MessageBase<M>
}

pub enum MessageBase<M: MsgTrait> {
  OneWay(M),
  Ask(M)
}

pub trait Dispatcher<M: MsgTrait, E: Error> {
  fn stop(&mut self);
  fn join(self) -> Result<(), E>;
}

pub struct AsyncDispatcher<M: MsgTrait, E: Error> {
  actors: Arc<Mutex<Vec<Box<Actor<M, E>>>>>,
  queue: Arc<MsQueue<M>>,
  stopped: Arc<Mutex<bool>>,
  thread: JoinHandle<Result<(), E>>,
}

unsafe impl<M: MsgTrait, E: Error> Sync for AsyncDispatcher<M, E> {}
unsafe impl<M: MsgTrait, E: Error> Send for AsyncDispatcher<M, E> {}

impl<M: MsgTrait, E: Error> AsyncDispatcher<M, E> {
  
  pub fn new() -> AsyncDispatcher<M, E> {
    let actors = Arc::new(Mutex::new(Vec::new()));
    let queue = Arc::new(MsQueue::new());
    let stopped = Arc::new(Mutex::new(false));
    let sleep_time = Duration::from_millis(50);

    AsyncDispatcher {
      actors: actors.clone(),
      queue: queue.clone(),
      stopped: stopped.clone(),
      thread: run(stopped, queue, actors),
    }
  }  

  pub fn send(&self, m: M) {
    self.queue.push(m);
  }
}

impl<M: MsgTrait, E: Error> Dispatcher<M, E> for AsyncDispatcher<M, E> {
  fn stop(&mut self) {
    debug!("stop enter");
    let mut stopped = self.stopped.lock().unwrap();
    *stopped = true;
    debug!("stop leave");
  }

  fn join(self) -> Result<(), E> {
    self.thread.join().unwrap()
  }
}

pub fn run<M, E>(stop: Arc<Mutex<bool>>, queue: Arc<MsQueue<M>>,
    actors: Arc<Mutex<Vec<Box<Actor<M, E>>>>>) -> JoinHandle<Result<(), E>>
    where M: MsgTrait, E: Error {

  let sleep_time = Duration::from_millis(50);

  thread::spawn(move || -> Result<(), E> {
     
     loop {
        if let Some(m) = queue.try_pop() {
          for actor in actors.lock().unwrap().iter_mut().filter(|x| x.accept(&m)) {
            match actor.on_receive(&m) {
              Err(e) => return Err(e),
              _ => {}
            }
          }
       } else {
         thread::sleep(sleep_time);
        }

        if *stop.lock().unwrap() == true {
          break;
        }
      }

      Ok(())
  })
}