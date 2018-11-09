use std::cell::RefCell;
use std::collections::HashMap;

use events::{SubmitResponseEvent, SystemEvents};
use redis_state_manager::RedisState;
use state::*;

pub fn submit_response(
    session_state: &mut HashMap<String, SessionState>,
    ev: SubmitResponseEvent,
) -> () {
    match session_state.clone().get(&ev.session_id) {
        Some(s) => {
            let prev_state: SessionState = s.clone();
            if prev_state.accepting_new_submissions {
                let mut session_events = prev_state.session_events.clone();
                session_events.push(SystemEvents::SubmitResponseEvent(ev.clone()));
                let next_state = SessionState {
                    session_events,
                    ..prev_state
                };

                session_state.insert(ev.session_id, next_state.clone());
                println!("[info] next state: {:?}", next_state);
            }
        }
        None => {
            println!("[warn] invalid id");
        }
    }
}

pub fn _submit_response(r_state: &RedisState, ev: SubmitResponseEvent) -> () {
    // TODO: For now, let's just save the events and not do anything
    println!("REDIS RESPONSE!");
    r_state
        .save_event(ev.clone(), vec![ev.session_id.clone()])
        .unwrap();
    let evs = r_state.read_events::<SubmitResponseEvent>(ev.session_id.clone());
    println!("Session started events: {:?}", evs,);
}
