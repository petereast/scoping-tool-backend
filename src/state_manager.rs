use mpsc::Receiver;
use std::collections::HashMap;
use std::thread;

use event_handlers::*;
use events::*;
use redis_state_manager::*;

pub fn start_state_manager(events_incoming_recv: Receiver<SystemEvents>) {
    thread::spawn(move || {
        let mut session_state = HashMap::new();
        loop {
            let incoming_event = events_incoming_recv
                .recv()
                .expect("can't get incoming event [DEPR]");
            // This event is coming from inside the system, but will be copied to be
            // persisted
            match incoming_event {
                SystemEvents::StartNewSessionEvent(e) => {
                    start_new_session(&mut session_state, e);
                }
                SystemEvents::EndSessionEvent(e) => end_session(&mut session_state, e),
                SystemEvents::SubmitResponseEvent(e) => {
                    submit_response(&mut session_state, e);
                }
                SystemEvents::GetSessionDetails(e) => {
                    get_session_details(&session_state, e);
                }
                SystemEvents::GetResponseCount(e) => {
                    get_response_count(&session_state, e);
                }
                SystemEvents::GetSessionResult(e) => {
                    get_session_result(&mut session_state, e);
                }
            }
        }
    });
}

// A redis state manager
pub fn _start_state_manager() {
    // Spawn a handler
    let state = RedisState::new("root".into());

    thread::spawn(move || {
        let local_state = RedisState::new("start_new_session_events".into());
        let start_new_session_events: EventStream<StartNewSessionEvent> =
            RedisState::get_queue_iter(&local_state, "scoping.StartNewSession".into());

        for ev in start_new_session_events {
            _start_new_session(&local_state, ev);
        }
    });

    thread::spawn(move || {
        let local_state = RedisState::new("new_submission_events".into());
        let submit_response_events: EventStream<SubmitResponseEvent> =
            RedisState::get_queue_iter(&local_state, "scopify.SubmitResponse".into());
        for ev in submit_response_events {
            _submit_response(&local_state, ev);
        }
    });
}
