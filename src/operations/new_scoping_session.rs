use actix_web::{AsyncResponder, Error, HttpResponse, Json, State};
use futures::future::{ok as FutOk, Future};

use events::*;
use http_interface::*;
use state::*;
use utils::generate_id;

pub fn new_scoping_session(
    (payload, state): (Json<NewScopingSessionCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    println!("[Request] new_scoping_session: {:?}", payload);
    let session_id = generate_id();

    state
        .outgoing_events
        .send(SystemEvents::StartNewSessionEvent(StartNewSessionEvent {
            session_id: session_id.clone().into(),
            session_title: payload.title.clone().into(),
            session_description: payload.description.clone().into(),
        })).unwrap();

    let submission_url = format!("https://localhost:4200/scope/{}", session_id);
    let response =
        NewScopingSessionOkResponse::new(session_id, submission_url.into(), "BOTTOM TEXT".into());

    println!("[Response] Ok: {:?}", response);
    FutOk(HttpResponse::Ok().json(response)).responder()
}
