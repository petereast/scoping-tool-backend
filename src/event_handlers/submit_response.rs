use std::collections::HashMap;

use events::{SubmitResponseEvent, SystemEvents};
use state::*;

pub fn submit_response(
    mut session_state: HashMap<String, SessionState>,
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
