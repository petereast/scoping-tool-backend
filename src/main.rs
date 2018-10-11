extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate json;

mod events;
mod http_interface;
mod operations;
mod state;
mod utils;

use actix_web::{
    http, server::HttpServer, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse,
    Json, Path, State,
};
use events::*;
use futures::future::{ok as FutOk, Future};
use http_interface::*;
use operations::*;
use state::*;
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::sync::Mutex;
use std::thread;

fn main() {
    let sys = actix::System::new("web");

    let (outgoing_events_sender, events_incoming_recv) = mpsc::sync_channel(10);

    let session_state = Mutex::new(HashMap::new());
    thread::spawn(move || loop {
        match events_incoming_recv.recv().unwrap() {
            SystemEvents::StartNewSessionEvent(e) => {
                session_state.lock().unwrap().insert(
                    e.session_id,
                    SessionState::new(e.session_title, e.session_description),
                );
            }
            SystemEvents::EndSessionEvent(e) => {
                match session_state.lock().unwrap().get(&e.session_id) {
                    Some(s) => {
                        let next_state = SessionState {
                            accepting_new_submissions: false,
                            ..(*s).clone()
                        };
                        session_state
                            .lock()
                            .unwrap()
                            .insert(e.session_id, next_state.clone());
                        println!("[info] next state: {:?}", next_state);
                    }
                    None => {
                        println!("[warn] invalid id");
                    }
                };
            }
            SystemEvents::SubmitResponseEvent(e) => {
                match session_state.lock().unwrap().get(&e.session_id) {
                    Some(s) => {
                        let prev_state: SessionState = s.clone();
                        if prev_state.accepting_new_submissions {
                            let mut session_events = prev_state.session_events.clone();
                            session_events.push(SystemEvents::SubmitResponseEvent(e.clone()));
                            let next_state = SessionState {
                                session_events,
                                ..prev_state
                            };

                            session_state
                                .lock()
                                .unwrap()
                                .insert(e.session_id, next_state.clone());
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

    HttpServer::new(move || {
        App::with_state(AppState {
            outgoing_events: outgoing_events_sender.clone(),
        }).resource("/health", |r| r.f(|_| HttpResponse::Ok().body("pppop")))
        .resource("/create-new-session", |r| {
            r.method(http::Method::POST).with(new_scoping_session)
        }).resource("/end-session", |r| {
            r.method(http::Method::POST).with(end_session)
        }).resource("/submit", |r| {
            r.method(http::Method::POST).with(submit_response)
        }).resource("/get-session-details/{id}", |r| {
            r.method(http::Method::GET).with(get_session_details)
        })
    }).bind("127.0.0.1:8008")
    .unwrap()
    .start();

    println!("Starting web service");
    let _ = sys.run();
}
