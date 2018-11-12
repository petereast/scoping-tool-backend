use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use aggregators::get_session_details as hydrate_session_details;
use futures::future::{ok as FutOk, Future};

use http_interface::*;
use state::*;

pub fn get_session_details(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    state
        .logger
        .log(format!("[Request] get_session_details: {:?}", get_path));
    let session = hydrate_session_details(&state.redis, get_path.id.clone());

    match session {
        Some(r) => {
            if !r.is_ended {
                FutOk(HttpResponse::Ok().json(GetSessionDetailsOkResponse {
                    title: r.title,
                    description: r.description,
                    session_id: get_path.id.clone(),
                    is_ended: r.is_ended,
                })).responder()
            } else {
                FutOk(
                    HttpResponse::TemporaryRedirect()
                        .header("Location", "/app/results/{}")
                        .body("Redirecting..."),
                ).responder()
            }
        }
        None => FutOk(HttpResponse::NotFound().body("not_found")).responder(),
    }
}
