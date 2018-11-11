use std::collections::HashMap;

use events::{GetSessionResult, GetSessionResultResponse, SubmissionContent, SystemEvents};
use state::*;

// This code should never run again!
pub fn _get_session_result(
    session_state: &mut HashMap<String, SessionState>,
    ev: GetSessionResult,
) -> () {
    ev.responder
        .send(match session_state.clone().get(&ev.session_id) {
            Some(state) => {
                let responses: Vec<SubmissionContent> =
                    aggregate_responses(session_state, ev.session_id);

                let average_response = match responses.get(0) {
                    Some(initial_value) => responses
                        .clone()
                        .split_off(1)
                        .iter()
                        .fold(initial_value.value, |acc, response| {
                            (acc + response.value) / 2
                        }),
                    None => 0,
                };

                Ok(GetSessionResultResponse {
                    title: state.title.clone(),
                    description: state.description.clone(),
                    response_count: responses.len(),
                    responses: responses,
                    average_response,
                })
            }
            None => Err(()),
        }).unwrap();
}

fn aggregate_responses(
    session_state: &HashMap<String, SessionState>,
    session_id: String,
) -> Vec<SubmissionContent> {
    if let Some(state) = session_state.get(&session_id) {
        state
            .session_events
            .iter()
            .filter_map(|ev| match ev {
                SystemEvents::SubmitResponseEvent(submission) => Some(SubmissionContent {
                    name: submission.name.clone(),
                    value: submission.value.clone(),
                }),
                _ => None,
            }).collect()
    } else {
        Vec::new()
    }
}
