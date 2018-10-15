use std::collections::HashMap;

use events::{GetSessionResult, GetSessionResultResponse, SubmissionContent, SystemEvents};
use state::*;

pub fn get_session_result(
    mut session_state: HashMap<String, SessionState>,
    ev: GetSessionResult,
) -> () {
    ev.responder
        .send(match session_state.clone().get(&ev.session_id) {
            Some(state) => {
                let responses: Vec<SubmissionContent> = state
                    .session_events
                    .iter()
                    .filter_map(|ev| match ev {
                        SystemEvents::SubmitResponseEvent(submission) => Some(SubmissionContent {
                            name: submission.name.clone(),
                            value: submission.value.clone(),
                        }),
                        _ => None,
                    }).collect();

                let average_response = match responses.get(0) {
                    Some(initial_value) => {
                        responses.iter().fold(initial_value.value, |acc, response| {
                            (acc + response.value) / 2
                        })
                    }
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
