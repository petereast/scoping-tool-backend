use std::collections::HashMap;

use events::{GetResponseCount, SystemEvents};
use state::*;

pub fn get_response_count(
    mut session_state: HashMap<String, SessionState>,
    ev: GetResponseCount,
) -> () {
    ev.responder
        .send(match session_state.clone().get(&ev.session_id) {
            Some(state) => {
                let names = state
                    .session_events
                    .iter()
                    .filter_map(|ev| match ev {
                        SystemEvents::SubmitResponseEvent(submission) => {
                            Some(submission.name.clone())
                        }
                        _ => None,
                    }).collect();
                Ok(names)
            }
            None => Err(()),
        }).unwrap();
}
