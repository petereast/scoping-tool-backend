use std::collections::HashMap;

use events::StartNewSessionEvent;
use state::*;

pub fn start_new_session(
    mut session_state: HashMap<String, SessionState>,
    ev: StartNewSessionEvent,
) -> () {
    session_state.insert(
        ev.session_id,
        SessionState::new(ev.session_title, ev.session_description),
    );
}
