use actix_web::{AsyncResponder, Error, HttpResponse, Json, State};
use futures::future::{ok as FutOk, Future};

use events::*;
use http_interface::*;
use state::*;

pub fn end_session(
    (payload, state): (Json<EndSessionCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    state
        .logger
        .log(format!("[Request] end_session: {:?}", payload));

    state
        .redis
        .emit(
            EndSessionEvent::new(payload.id.clone().into()),
            "scoping.EndSession".into(),
        ).expect("Couldn't emit EndSessionEvent");

    FutOk(
        HttpResponse::Ok()
            .content_type("application/json")
            .body("{\"ok\": true}"),
    ).responder()
}
