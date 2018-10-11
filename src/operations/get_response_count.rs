use actix_web::{AsyncResponder, Error, HttpResponse, Path, State};
use futures::future::{ok as FutOk, Future};
use mpsc::sync_channel;

use events::*;
use http_interface::*;
use state::*;

pub fn get_response_count(
    (get_path, state): (Path<GetSessionDetailsCmd>, State<AppState>),
) -> Box<Future<Item = HttpResponse, Error = Error>> {
    println!("[Request] get_response_count: {:?}", get_path);

    let (responder, recv) = sync_channel(1);

    let outgoing_event = GetResponseCount {
        session_id: get_path.id.clone(),
        responder,
    };

    state
        .outgoing_events
        .send(SystemEvents::GetResponseCount(outgoing_event))
        .unwrap();

    let data_response = recv.recv().unwrap();

    println!("thing: {:?}", data_response);

    match data_response {
        Ok(r) => FutOk(HttpResponse::Ok().json(GetResponseCountOkResponse {
            count: r.len(),
            names: r,
            session_id: get_path.id.clone(),
        })).responder(),
        Err(_) => FutOk(HttpResponse::NotFound().body("not_found")).responder(),
    }
}
