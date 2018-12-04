use std::thread;

use event_handlers::*;
use events::*;
use redis_state_manager::*;

pub fn start_event_listeners() {
    thread::spawn(move || {
        let local_state = RedisState::new("start_new_session_events".into());
        let start_new_session_events: EventStream<StartNewSessionEvent> =
            RedisState::get_queue_iter(&local_state, "scoping.StartNewSession".into());

        for ev in start_new_session_events {
            start_new_session(&local_state, ev);
        }
    });

    thread::spawn(move || {
        let local_state = RedisState::new("new_submission_events".into());
        let submit_response_events: EventStream<SubmitResponseEvent> =
            RedisState::get_queue_iter(&local_state, "scopify.SubmitResponse".into());
        for ev in submit_response_events {
            submit_response(&local_state, ev);
        }
    });

    thread::spawn(move || {
        let local_state = RedisState::new("end_session_events".into());
        let end_session_events: EventStream<EndSessionEvent> =
            RedisState::get_queue_iter(&local_state, "scopify.EndSession".into());
        for ev in end_session_events {
            end_session(&local_state, ev);
        }
    });
}
