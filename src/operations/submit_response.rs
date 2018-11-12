use actix_web::{AsyncResponder, Error, HttpResponse, Json, State};
use futures::future::{ok as FutOk, Future};

use events::*;
use http_interface::*;
use state::*;

pub fn submit_response(
    (payload, state): (Json<SubmitResponseCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    state
        .redis
        .emit(
            SubmitResponseEvent::new(
                payload.session_id.clone().into(),
                payload.name.clone().into(),
                payload.value.clone().into(),
            ),
            "scopify.SubmitResponse".into(),
        ).expect("Can't emit SubmitResponseEvent");

    state
        .logger
        .log(format!("[Request] submit_response: {:?}", payload));
    FutOk(
        HttpResponse::Ok()
            .content_type("application/json")
            .body("{\"ok\": true}"),
    ).responder()
}
