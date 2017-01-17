
pub trait Router<M: 'static + Sync + Send + Decodable, E: 'static + Sized + Sync + Send> {
  fn tx(&self, m: &M) -> Result<(), E>;
  fn rx(&self) -> Result<M, E>;
}