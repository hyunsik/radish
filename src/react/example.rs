
pub enum Payload<M> {
  Message<M>,
  Ask<M>
}

pub enum NodeProtocol {
  UpdateRequest
  UpdateResponse  
}

pub enum NodeErr {
  
}

pub struct NodeManager {
  eb: &EventBus
}

impl NodeManager for Actor {
  fn on_ask(&self, msg: &NodeProtocol) -> Result<NodeProtocol, NodeErr> {
    match msg {
      Update => Ok(build_update())
      _ => panic!("Unknown")
    }
  }

  fn on_receive(&self, msg: &NodeProtocol) -> Result<(), NodeErr> {
  }
}

pub struct NodeLeader for Actor {
  fn on_ask(&self, msg: &NodeProtocol) -> Result<NodeProtocol, NodeErr> {
    match msg {
      Update => Ok(build_update())
      _ => panic!("Unknown")
    }
  }

  fn on_receive(&self, msg: &NodeProtocol) -> Result<(), NodeErr> {
  }
}

pub fn main() {
  let actor_system = ActorSystem::with_config("actor1", "192.168.0.1", 8888, "/react/members");
  let event_bus = actor_system.new_event_bus();
  
  let actor1 = Actor::new(_);  
  let actor2 = Actor::new(_);

  event_bus.subscribe(actor1);
  event_bus.subscribe(actor2);

  loop {    
    thread:sleep(1000);
    event_bus.send(Update)
  }
}