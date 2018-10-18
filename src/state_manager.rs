use mpsc::{Receiver, SyncSender};
use std::collections::HashMap;
use std::thread;

use event_handlers::*;
use events::*;

pub fn start_state_manager(events_incoming_recv: Receiver<SystemEvents>) {
    thread::spawn(move || {
        let mut session_state = HashMap::new();
        loop {
            let incoming_event = events_incoming_recv.recv().unwrap();
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

pub fn state_persister(events_outgoing_bus: Receiver<SystemEvents>) {
    // Spawn a thread to handle persisting events
}

pub fn state_sync(incoming_events_bus: SyncSender<SystemEvents>) {
    // Spawn a thread to bring in events from outside - be it through
    // polling changes in the db or by subscribing to a listener
}
