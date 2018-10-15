use std::collections::HashMap;

use events::EndSessionEvent;
use state::*;

pub fn end_session(session_state: &mut HashMap<String, SessionState>, ev: EndSessionEvent) -> () {
    match session_state.clone().get(&ev.session_id) {
        Some(s) => {
            let next_state = SessionState {
                accepting_new_submissions: false,
                ..(*s).clone()
            };
            session_state.insert(ev.session_id, next_state.clone());
            println!("[info] next state: {:?}", next_state);
        }
        None => {
            println!("[warn] invalid id");
        }
    };
}
