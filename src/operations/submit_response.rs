use actix_web::{AsyncResponder, Error, HttpResponse, Json, State};
use futures::future::{ok as FutOk, Future};

use events::*;
use http_interface::*;
use state::*;

pub fn submit_response(
    (payload, state): (Json<SubmitResponseCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    println!("[Request] submit_response: {:?}", payload);
    // Save a response against a session

    state
        .outgoing_events
        .send(SystemEvents::SubmitResponseEvent(SubmitResponseEvent {
            session_id: payload.session_id.clone().into(),
            name: payload.name.clone().into(),
            value: payload.value.clone().into(),
        })).unwrap();

    FutOk(
        HttpResponse::Ok()
            .content_type("application/json")
            .body("{\"ok\": true}"),
    ).responder()
}
