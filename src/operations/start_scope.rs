use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use mpsc::sync_channel;

use events::*;
use http_interface::*;
use state::*;

pub fn start_scope(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    state
        .logger
        .log(format!("[Request] start_scope_response: {:?}", get_path));

    let (responder, recv) = sync_channel(1);

    let outgoing_event = GetSessionDetails {
        session_id: get_path.id.clone(),
        responder,
    };

    state
        .outgoing_events
        .send(SystemEvents::GetSessionDetails(outgoing_event))
        .unwrap();

    let data_response = recv.recv().unwrap();

    println!("thing: {:?}", data_response);

    // If the session is ended, redirect the user to the results page.

    match data_response {
        Ok(r) => {
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
        Err(_) => FutOk(
            HttpResponse::TemporaryRedirect()
                .header("Location", "/")
                .finish(),
        ).responder(),
    }
}
