use std::collections::HashMap;

use events::{GetSessionDetails, GetSessionDetailsResponse};
use state::*;

pub fn get_session_details(
    session_state: &HashMap<String, SessionState>,
    ev: GetSessionDetails,
) -> () {
    ev.responder
        .send(match session_state.clone().get(&ev.session_id) {
            Some(state) => Ok(GetSessionDetailsResponse {
                title: state.title.clone(),
                description: state.description.clone(),
                is_ended: !state.accepting_new_submissions,
            }),
            None => Err(()),
        }).unwrap();
}
