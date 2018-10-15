use mpsc::Receiver;
use std::collections::HashMap;
use std::thread;

use event_handlers::*;
use events::*;

pub fn start_state_manager(events_incoming_recv: Receiver<SystemEvents>) {
    let mut session_state = HashMap::new();
    thread::spawn(move || loop {
        match events_incoming_recv.recv().unwrap() {
            SystemEvents::StartNewSessionEvent(e) => {
                start_new_session(session_state, e);
            }
            SystemEvents::EndSessionEvent(e) => end_session(session_state, e),
            SystemEvents::SubmitResponseEvent(e) => {
                submit_response(session_state, e);
            }
            SystemEvents::GetSessionDetails(e) => {
                get_session_details(session_state, e);
            }
            SystemEvents::GetResponseCount(e) => {
                get_response_count(session_state, e);
            }
            SystemEvents::GetSessionResult(e) => {
                get_session_result(session_state, e);
            }
            _ => {
                println!("System events");
            }
        }
    });
}
