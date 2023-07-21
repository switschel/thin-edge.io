use std::time::Duration;
use tedge_actors::Message;

/// Request a timeout to be set
///
/// After the given duration the timer will return the event back to the caller.
#[derive(Debug)]
pub struct SetTimeout<T: Message> {
    pub duration: Duration,
    pub event: T,
}

impl<T: Message> SetTimeout<T> {
    pub fn new(duration: Duration, event: T) -> Self {
        Self { duration, event }
    }
}

/// Timeout event sent by the timer back to the caller
#[derive(Debug)]
pub struct Timeout<T: Message> {
    pub event: T,
}

impl<T: Message> Timeout<T> {
    pub fn new(event: T) -> Self {
        Self { event }
    }
}

impl<T: Message + Eq + PartialEq> PartialEq<Self> for Timeout<T> {
    fn eq(&self, other: &Self) -> bool {
        self.event.eq(&other.event)
    }
}

impl<T: Message + Eq + PartialEq> Eq for Timeout<T> {}
