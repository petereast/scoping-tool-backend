use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct TransportResponse<T: Serialize> {
    ev: T,
    // The name of where the response is going to be put
    response_queue: String,
}

pub struct RedisState {}

impl RedisState {
    pub fn _new() -> Self {
        Self {}
    }
    pub fn _emit(ev: String, queue_name: String) -> Result<String, String> {
        let response_key = Uuid::new_v4().to_string();
        // TODO: Do some enums on the event queue name
        // Return the response id
        let transport_payload = TransportResponse {
            ev,
            response_queue: response_key.to_owned(),
        };

        // Send a redis message to queue_name
        Ok(response_key)
    }
}

// Do I want to use callbacks OR do I want to provide a function that blocks until an event arrives
//
// BRPOPLPUSH - from incoming events and into a 'to store' queue
// or
// BRPOP - from incoming events and store as part of the handler
// or
// BRPOP an event from a queue to act on it, then pub it somewhere to store it
//
// pub fn subscribe(queue_name: String, callback: Fn() -> Result<Option<String>, String>) {}

// TODO: There should be an enum of usable queues
// TODO: Also there should be a wrapper around the returned event

pub fn _get_next_event(event_name: String, store: bool) -> Result<String, ()> {
    if store {
        println!("{}", event_name);
    }
    Err(())
}

// Returns the id of the queue the data is going to be stored in
// Serializable s :: s -> String
pub fn _send_response(data: String) -> String {
    let response_id = Uuid::new_v4();

    // Spawn a thread to perform the IO operation asynchronously

    response_id.clone().to_string()
}
