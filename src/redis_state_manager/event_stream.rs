use redis_state_manager::redis_state::RedisState;
use serde::{de::DeserializeOwned, Serialize};

pub struct EventStream<T: DeserializeOwned> {
    queue_name: String,
    state: Box<RedisState>,
    pub curr: Option<T>,
}

impl<T: DeserializeOwned + Serialize> Iterator for EventStream<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.state
            .get_next_incoming_event(self.queue_name.clone())
            .map(|wrapped| wrapped.ev)
    }
}

impl<T: DeserializeOwned> EventStream<T> {
    pub fn new(queue_name: String, state: Box<RedisState>) -> Self {
        Self {
            queue_name,
            state,
            curr: None,
        }
    }
}
