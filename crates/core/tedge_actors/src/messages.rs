use std::fmt::Debug;

/// A message exchanged between two actors
pub trait Message: Debug + Send + Sync + 'static {}

/// There is no need to tag messages as such
impl<T: Debug + Send + Sync + 'static> Message for T {}

/// A type to tell no message is received or sent
#[derive(Debug)]
pub enum NoMessage {}
