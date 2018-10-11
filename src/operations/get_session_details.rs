use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};

use events::*;
use http_interface::*;
use serde_json::to_string;
use state::*;

pub fn get_session_details(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    println!("[Request] submit_response: {:?}", get_path);

    let response = GetSessionDetailsOkResponse {
        title: "".into(),
        description: "".into(),
        session_id: get_path.id.clone(),
    };

    // Trigger a state request with an event that contains a channel sender
    // wait for the response from the new channel

    FutOk(
        HttpResponse::Ok()
            .content_type("application/json")
            .body(to_string(&response).unwrap()),
    ).responder()
}
