use mpsc::Receiver;
use std::collections::HashMap;
use std::thread;

use event_handlers::*;
use events::*;
use redis_state_manager::*;

pub fn start_state_manager(events_incoming_recv: Receiver<SystemEvents>) {
    thread::spawn(move || {
        let session_state = HashMap::new();
        println!("WARN: Depricated!! Using old state manager");
        loop {
            let incoming_event = events_incoming_recv
                .recv()
                .expect("can't get incoming event [DEPR]");
            // This event is coming from inside the system, but will be copied to be
            // persisted
            match incoming_event {
                SystemEvents::GetSessionDetails(e) => {
                    get_session_details(&session_state, e);
                }
                SystemEvents::GetResponseCount(e) => {
                    get_response_count(&session_state, e);
                }
                _ => (),
            }
        }
    });
}

// A redis state manager
pub fn _start_state_manager() {
    // Spawn a handler
    let _state = RedisState::new("root".into());

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
            _submit_response(&local_state, ev);
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
