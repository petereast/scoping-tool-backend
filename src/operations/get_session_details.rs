use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use mpsc::sync_channel;

use events::*;
use http_interface::*;
use state::*;

pub fn get_session_details(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    println!("[Request] submit_response: {:?}", get_path);

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

    match data_response {
        Ok(r) => FutOk(HttpResponse::Ok().json(GetSessionDetailsOkResponse {
            title: r.title,
            description: r.description,
            session_id: get_path.id.clone(),
        })).responder(),
        Err(_) => FutOk(HttpResponse::NotFound().body("not_found")).responder(),
    }
}