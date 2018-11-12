use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use aggregators::get_session_details;
use futures::future::{ok as FutOk, Future};
use http_interface::*;
use state::*;

pub fn start_scope(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    state
        .logger
        .log(format!("[Request] start_scope_response: {:?}", get_path));

    let session = get_session_details(&state.redis, get_path.id.clone());

    match session {
        Some(r) => {
            if !r.is_ended {
                FutOk(
                    HttpResponse::TemporaryRedirect()
                        .header("Location", format!("/app/scope/{}", get_path.id))
                        .finish(),
                ).responder()
            } else {
                FutOk(
                    HttpResponse::TemporaryRedirect()
                        .header("Location", format!("/app/results/{}", get_path.id))
                        .finish(),
                ).responder()
            }
        }
        None => FutOk(
            HttpResponse::TemporaryRedirect()
                .header("Location", "/")
                .finish(),
        ).responder(),
    }
}
