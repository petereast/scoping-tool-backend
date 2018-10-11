use mpsc::Receiver;
use std::collections::HashMap;
use std::thread;

use events::*;
use state::*;

pub fn start_state_manager(events_incoming_recv: Receiver<SystemEvents>) {
    thread::spawn(move || loop {
        let mut session_state = HashMap::new();
        match events_incoming_recv.recv().unwrap() {
            SystemEvents::StartNewSessionEvent(e) => {
                session_state.insert(
                    e.session_id,
                    SessionState::new(e.session_title, e.session_description),
                );
            }
            SystemEvents::EndSessionEvent(e) => {
                match session_state.clone().get(&e.session_id) {
                    Some(s) => {
                        let next_state = SessionState {
                            accepting_new_submissions: false,
                            ..(*s).clone()
                        };
                        session_state.insert(e.session_id, next_state.clone());
                        println!("[info] next state: {:?}", next_state);
                    }
                    None => {
                        println!("[warn] invalid id");
                    }
                };
            }
            SystemEvents::SubmitResponseEvent(e) => {
                match session_state.clone().get(&e.session_id) {
                    Some(s) => {
                        let prev_state: SessionState = s.clone();
                        if prev_state.accepting_new_submissions {
                            let mut session_events = prev_state.session_events.clone();
                            session_events.push(SystemEvents::SubmitResponseEvent(e.clone()));
                            let next_state = SessionState {
                                session_events,
                                ..prev_state
                            };

                            session_state.insert(e.session_id, next_state.clone());
                            println!("[info] next state: {:?}", next_state);
                        }
                    }
                    None => {
                        println!("[warn] invalid id");
                    }
                }
            }
            _ => {
                println!("System events");
            }
        }
    });
}
