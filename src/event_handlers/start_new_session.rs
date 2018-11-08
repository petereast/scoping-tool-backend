use std::collections::HashMap;

use events::StartNewSessionEvent;
use redis_state_manager::*;
use state::*;

pub fn start_new_session(
    session_state: &mut HashMap<String, SessionState>,
    ev: StartNewSessionEvent,
) -> () {
    session_state.insert(
        ev.session_id,
        SessionState::new(ev.session_title, ev.session_description),
    );
}

pub fn _start_new_session(r_state: &RedisState, ev: StartNewSessionEvent) -> () {
    // Do a thing
    // Save this event into the store and process i
    println!("New session REDIS!");
    r_state
        .save_event(ev.clone(), vec![ev.session_id.clone()])
        .unwrap();

    let evs = r_state.read_events::<StartNewSessionEvent>(ev.session_id.clone());
    println!("{:?}", evs,);
}
