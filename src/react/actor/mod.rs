use super::{MsgTrait, Error};

pub struct ActorUri {
  host_name: String,
  port: i32,
  path: String
}

impl ActorUri {
  pub fn display(&self) -> String {
    format!("react://{}:{}/{}", self.host_name, self.port, self.path)
  }
}

pub struct ActorContext {
  uri: ActorUri  
}

pub trait Actor<M: MsgTrait, E: Error>: Send + Sync {
  fn context(&self) -> &ActorContext;
  fn accept(&self, m: &M) -> bool { true }
  fn on_receive(&mut self, m: &M) -> Result<(), E>;
}