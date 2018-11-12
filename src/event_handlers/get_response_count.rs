use std::collections::HashMap;

use state::*;

pub fn get_response_count(
    session_state: &HashMap<String, SessionState>,
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
                let response = GetResponseCountResponse {
                    names,
                    is_ended: !state.accepting_new_submissions,
                };

                Ok(response)
            }
            None => Err(()),
        }).unwrap();
}
