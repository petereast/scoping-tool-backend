use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use mpsc::sync_channel;
use uuid::Uuid;

use aggregators::*;
use events::*;
use http_interface::*;
use state::*;

pub fn get_session_result(
    (get_path, state): (Path<GetSessionResultCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    state
        .logger
        .log(format!("[Request] get_session_details: {:?}", get_path));

    let session_id = get_path.id.clone();

    let _ = state
        .redis
        .emit(
            _GetSessionResult {
                session_id: session_id.clone(),
            },
            "scopify.GetSessionResult".into(),
        ).expect("Can't emit GetSessionResult");

    let data_response = hydrate_session_result(&state.redis, session_id.clone());

    println!("Redis response: {:?}", data_response);

    match data_response {
        Some(r) => {
            let average_response = match r.responses.get(0) {
                Some(initial_value) => r
                    .responses
                    .clone()
                    .split_off(1)
                    .iter()
                    .fold(initial_value.value, |acc, response| {
                        (acc + response.value) / 2
                    }),
                None => 0,
            };
            FutOk(HttpResponse::Ok().json(GetSessionResultOkResponse {
                title: r.title,
                description: r.description,
                average_response: average_response,
                response_count: r.response_count,
                responses: r.responses,
            })).responder()
        }
        None => FutOk(HttpResponse::NotFound().body("not_found")).responder(),
    }
}
