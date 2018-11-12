use redis_state_manager::redis_state::RedisState;
use serde::{de::DeserializeOwned, Serialize};

pub struct EventStream<'a, T: DeserializeOwned> {
    queue_name: String,
    state: &'a RedisState,
    pub curr: Option<T>,
}

impl<'a, T: DeserializeOwned + Serialize> Iterator for EventStream<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.state
            .get_next_incoming_event(self.queue_name.clone())
            .map(|wrapped| wrapped.ev)
    }
}

impl<'a, T: DeserializeOwned> EventStream<'a, T> {
    pub fn new(queue_name: String, state: &'a RedisState) -> Self {
        Self {
            queue_name,
            state,
            curr: None,
        }
    }
}
